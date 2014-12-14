#![feature(unboxed_closures)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void};
use std::sync::{Arc, Weak};
use std::collections::HashMap;
use std::cell::UnsafeCell;
use std::c_str::CString;

enum QQuickView {}
enum QVariant {}

#[repr(C)]
#[deriving(Eq, PartialEq, Show, Copy)]
enum QrsVariantType {
    Invalid = 0,
    Int
}

extern "C" {
    fn qmlrs_create_view() -> *mut QQuickView;
    fn qmlrs_destroy_view(view: *mut QQuickView);
    fn qmlrs_view_set_source(view: *mut QQuickView, path: *const c_char, len: c_uint);
    fn qmlrs_view_show(view: *mut QQuickView);
    fn qmlrs_view_invoke(view: *mut QQuickView, method: *const c_char, result: *mut QVariant,
                         n_args: c_uint, r_args: *const *const QVariant);
    fn qmlrs_view_set_slot_function(view: *mut QQuickView,
                                    fun: extern "C" fn(*const c_char, *mut c_void, *mut QVariant),
                                    data: *mut c_void);

    fn qmlrs_variant_create() -> *mut QVariant;
    fn qmlrs_variant_destroy(v: *mut QVariant);
    fn qmlrs_variant_set_int(var: *mut QVariant, x: c_int);
    fn qmlrs_variant_set_invalid(var: *mut QVariant);
    fn qmlrs_variant_get_type(var: *const QVariant) -> QrsVariantType;
    fn qmlrs_variant_get_int(var: *const QVariant, x: *mut c_int);

    fn qmlrs_app_exec();
}

#[deriving(Eq, PartialEq, Show, Copy)]
pub enum Variant {
    Invalid,
    Int(int)
}

impl Variant {
    fn set_into(self, var: *mut QVariant) {
        unsafe {
            match self {
                Variant::Invalid => qmlrs_variant_set_invalid(var),
                Variant::Int(x) => qmlrs_variant_set_int(var, x as c_int)
            }
        }
    }

    fn get_from(var: *const QVariant) -> Variant {
        unsafe {
            match qmlrs_variant_get_type(var) {
                QrsVariantType::Invalid => Variant::Invalid,
                QrsVariantType::Int => {
                    let mut x: c_int = 0;
                    qmlrs_variant_get_int(var, &mut x);
                    Variant::Int(x as int)
                }
            }
        }
    }
}

pub type Slot = Box<FnMut<(),Variant> + 'static>;

struct ViewInternal {
    p: *mut QQuickView,
    slots: UnsafeCell<HashMap<String, Slot>>
}

impl Drop for ViewInternal {
    fn drop(&mut self) {
        unsafe { qmlrs_destroy_view(self.p); }
    }
}

pub struct View {
    nosend: ::std::kinds::marker::NoSend,
    i: Arc<ViewInternal>
}

extern "C" fn slot_fun(slot: *const c_char, data: *mut c_void, result: *mut QVariant) {
    /* ViewInternal must be alive here, since the Qml context is alive */

    let i: &ViewInternal = unsafe { std::mem::transmute(data) };
    let cstr = unsafe { CString::new(slot, false) };

    unsafe {
        /* Must be UTF-8 since these are created from Rust code */
        match (*i.slots.get()).get_mut(cstr.as_str().unwrap()) {
            Some(slot) => slot.call_mut(()).set_into(result),
            None       => {
                println!("Warning: unregistered slot called from Qml");
                qmlrs_variant_set_invalid(result);
            }
        }
    }
}

impl View {
    pub fn new() -> View {
        let p = unsafe { qmlrs_create_view() };
        assert!(p.is_not_null());

        let i = Arc::new(ViewInternal {
            p: p,
            slots: UnsafeCell::new(HashMap::new())
        });

        unsafe {
            qmlrs_view_set_slot_function(p, slot_fun, i.deref() as *const ViewInternal
                                                                as *mut c_void);
        }

        View {
            nosend: ::std::kinds::marker::NoSend,
            i: i
        }
    }

    pub fn set_source(&mut self, path: &str) {
        unsafe {
            qmlrs_view_set_source(self.i.p, path.as_ptr() as *const c_char, path.len() as c_uint);
        }
    }

    pub fn show(&mut self) {
        unsafe {
            qmlrs_view_show(self.i.p);
        }
    }

    pub fn register_slot(&mut self, name: String, slot: Slot) {
        unsafe {
            (*self.i.slots.get()).insert(name, slot);
        }
    }

    pub fn exec(&mut self) {
        unsafe { qmlrs_app_exec(); }
    }

    pub fn handle(&self) -> Handle {
        Handle { i: self.i.downgrade() }
    }
}

pub struct Handle {
    i: Weak<ViewInternal>
}

impl Handle {
    pub fn invoke(&self, method: &str, args: &[Variant]) -> Result<Variant, &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            let c_args: Vec<*const QVariant> = args.iter().map(|v| {
                let cv = qmlrs_variant_create();
                assert!(cv.is_not_null());
                v.set_into(cv);
                cv as *const QVariant
            }).collect();

            let result = qmlrs_variant_create();
            assert!(result.is_not_null());

            match self.i.upgrade() {
                Some(i) => qmlrs_view_invoke(i.p, cstr.as_ptr(), result,
                                             c_args.len() as c_uint, c_args.as_ptr()),
                None    => { qmlrs_variant_destroy(result); return Err("View has been freed") }
            }

            let ret = Variant::get_from(result as *const QVariant);
            qmlrs_variant_destroy(result);

            Ok(ret)
        }
    }
}
