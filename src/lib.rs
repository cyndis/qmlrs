#![feature(unboxed_closures, globs, macro_rules)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void};
use std::sync::{Arc, Weak};

use ffi::{QVariant, QrsVariantType, QrsEngine, QVariantList};
pub mod ffi;
mod macro;

#[deriving(Eq, PartialEq, Show)]
pub enum Variant {
    Invalid,
    Int(int),
    String(String)
}

impl Variant {
    fn set_into(&self, var: *mut QVariant) {
        unsafe {
            match *self {
                Variant::Invalid => ffi::qmlrs_variant_set_invalid(var),
                Variant::Int(x) => ffi::qmlrs_variant_set_int(var, x as c_int),
                Variant::String(ref x) => ffi::qmlrs_variant_set_string(var, x.len() as c_uint,
                                                                   x.as_ptr() as *const c_char)
            }
        }
    }

    fn get_from(var: *const QVariant) -> Variant {
        unsafe {
            match ffi::qmlrs_variant_get_type(var) {
                QrsVariantType::Invalid => Variant::Invalid,
                QrsVariantType::Int => {
                    let mut x: c_int = 0;
                    ffi::qmlrs_variant_get_int(var, &mut x);
                    Variant::Int(x as int)
                },
                QrsVariantType::String => {
                    let mut len: c_uint = 0;
                    ffi::qmlrs_variant_get_string_length(var, &mut len);

                    let mut data: Vec<u8> = Vec::with_capacity(len as uint);
                    ffi::qmlrs_variant_get_string_data(var, data.as_mut_ptr() as *mut c_char);
                    data.set_len(len as uint);

                    Variant::String(String::from_utf8_unchecked(data))
                }
            }
        }
    }
}

//pub type Slot = Box<FnMut<(Vec<Variant>,),Variant> + 'static>;
pub type Slot = Box<FnMut<(), ()> + 'static>;

struct EngineInternal {
    p: *mut QrsEngine,
}

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { ffi::qmlrs_destroy_engine(self.p); }
    }
}

pub struct Engine {
    nosend: ::std::kinds::marker::NoSend,
    i: Arc<EngineInternal>
}

/*
extern "C" fn slot_fun(slot: *const c_char, data: *mut c_void, result: *mut QVariant,
                       c_args: *mut QVariantList)
{
    /* EngineInternal must be alive here, since the Qml context is alive */

    let i: &EngineInternal = unsafe { std::mem::transmute(data) };
    let cstr = unsafe { CString::new(slot, false) };

    unsafe {
        let mut args = vec![];
        for j in range(0, ffi::qmlrs_varlist_length(c_args as *const QVariantList)) {
            let c_arg = ffi::qmlrs_varlist_get(c_args as *const QVariantList, j);
            args.push(Variant::get_from(c_arg as *const QVariant));
        }

        /* Must be UTF-8 since these are created from Rust code */
        match (*i.slots.get()).get_mut(cstr.as_str().unwrap()) {
            Some(slot) => slot.call_mut((args,)).set_into(result),
            None       => {
                println!("Warning: unregistered slot called from Qml");
                ffi::qmlrs_variant_set_invalid(result);
            }
        }
    }
}
*/

extern "C" fn slot_handler<T: Object>(data: *mut c_void, slot: c_int,
                                      args: *const *const ffi::QVariant)
{
    unsafe {
        let obj: &mut T = std::mem::transmute(data);
        obj.qt_metacall(slot as i32);
    }
}

impl Engine {
    pub fn new() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine() };
        assert!(p.is_not_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            nosend: ::std::kinds::marker::NoSend,
            i: i
        }
    }

    pub fn load_url(&mut self, path: &str) {
        unsafe {
            ffi::qmlrs_engine_load_url(self.i.p, path.as_ptr() as *const c_char, path.len() as c_uint);
        }
    }

    pub fn exec(self) {
        unsafe { ffi::qmlrs_app_exec(); }
    }

    pub fn handle(&self) -> Handle {
        Handle { i: self.i.downgrade() }
    }

    pub fn set_property<T: Object>(&mut self, name: &str, obj: T) {
        unsafe {
            let mo = obj.qt_metaobject().p;
            let mut boxed = box obj;
            let qobj = ffi::qmlrs_metaobject_instantiate(mo, slot_handler::<T>,
                                                         &mut *boxed as *mut T as *mut c_void);

            ffi::qmlrs_engine_set_property(self.i.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, qobj);

            std::mem::forget(boxed);
        }
    }
}

pub trait Object {
    fn qt_metaobject(&self) -> MetaObject;
    fn qt_metacall(&mut self, slot: i32);
}

#[allow(missing_copy_implementations)]
pub struct MetaObject {
    p: *mut ffi::QrsMetaObject
}

impl MetaObject {
    pub fn new() -> MetaObject {
        let p = unsafe { ffi::qmlrs_metaobject_create() };
        assert!(p.is_not_null());

        MetaObject { p: p }
    }

    pub fn method(self, name: &str, argc: u8) -> MetaObject {
        unsafe {
            ffi::qmlrs_metaobject_add_slot(self.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, argc as c_uint);
        }
        self
    }
}

pub struct Handle {
    i: Weak<EngineInternal>
}

impl Handle {
    pub fn invoke(&self, method: &str, args: &[Variant]) -> Result<Variant, &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            let c_args = ffi::qmlrs_varlist_create();
            assert!(c_args.is_not_null());
            for arg in args.iter() {
                let c_arg = ffi::qmlrs_varlist_push(c_args);
                assert!(c_arg.is_not_null());
                arg.set_into(c_arg);
            }

            let result = ffi::qmlrs_variant_create();
            assert!(result.is_not_null());

            match self.i.upgrade() {
                Some(i) => ffi::qmlrs_engine_invoke(i.p, cstr.as_ptr(), result,
                                               c_args as *const QVariantList),
                None    => {
                    ffi::qmlrs_variant_destroy(result);
                    ffi::qmlrs_varlist_destroy(c_args);
                    return Err("View has been freed")
                }
            }

            ffi::qmlrs_varlist_destroy(c_args);

            let ret = Variant::get_from(result as *const QVariant);
            ffi::qmlrs_variant_destroy(result);

            Ok(ret)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_engine() {
        Engine::new();
    }
}
