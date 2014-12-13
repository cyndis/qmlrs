#![feature(unboxed_closures)]

extern crate libc;

use libc::{c_char, c_uint, c_void};
use std::sync::{Arc, Weak};
use std::collections::HashMap;
use std::cell::UnsafeCell;
use std::c_str::CString;

enum QQuickView {}

extern "C" {
    fn qmlrs_create_view() -> *mut QQuickView;
    fn qmlrs_destroy_view(view: *mut QQuickView);
    fn qmlrs_view_set_source(view: *mut QQuickView, path: *const c_char, len: c_uint);
    fn qmlrs_view_show(view: *mut QQuickView);
    fn qmlrs_view_invoke(view: *mut QQuickView, method: *const c_char);
    fn qmlrs_view_set_slot_function(view: *mut QQuickView,
                                    fun: extern "C" fn(*const c_char, *mut c_void),
                                    data: *mut c_void);

    fn qmlrs_app_exec();
}

pub type Slot = Box<FnMut<(),()> + 'static>;

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

extern "C" fn slot_fun(slot: *const c_char, data: *mut c_void) {
    /* ViewInternal must be alive here, since the Qml context is alive */

    let i: &ViewInternal = unsafe { std::mem::transmute(data) };
    let cstr = unsafe { CString::new(slot, false) };

    unsafe {
        /* Must be UTF-8 since these are created from Rust code */
        match (*i.slots.get()).get_mut(cstr.as_str().unwrap()) {
            Some(slot) => slot.call_mut(()),
            None       => println!("Warning: unregistered slot called from Qml")
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
    pub fn invoke(&self, method: &str) -> Result<(), &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            match self.i.upgrade() {
                Some(i) => qmlrs_view_invoke(i.p, cstr.as_ptr()),
                None    => return Err("View has been freed")
            }
        }

        Ok(())
    }
}
