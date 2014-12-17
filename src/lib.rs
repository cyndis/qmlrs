#![feature(unboxed_closures, globs)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void};
use std::sync::{Arc, Weak};
use std::collections::HashMap;
use std::cell::UnsafeCell;
use std::c_str::CString;
use std::borrow::ToOwned;

enum QrsEngine {}
enum QVariant {}
enum QVariantList {}

#[repr(C)]
#[deriving(Eq, PartialEq, Show, Copy)]
enum QrsVariantType {
    Invalid = 0,
    Int,
    String
}

extern "C" {
    fn qmlrs_create_engine() -> *mut QrsEngine;
    fn qmlrs_destroy_engine(engine: *mut QrsEngine);
    fn qmlrs_engine_load_url(engine: *mut QrsEngine, path: *const c_char, len: c_uint);
    fn qmlrs_engine_invoke(engine: *mut QrsEngine, method: *const c_char, result: *mut QVariant,
                           args: *const QVariantList);
    fn qmlrs_engine_set_slot_function(engine: *mut QrsEngine,
                                      fun: extern "C" fn(*const c_char, *mut c_void, *mut QVariant,
                                                         *mut QVariantList),
                                      data: *mut c_void);

    fn qmlrs_variant_create() -> *mut QVariant;
    fn qmlrs_variant_destroy(v: *mut QVariant);
    fn qmlrs_variant_set_int(var: *mut QVariant, x: c_int);
    fn qmlrs_variant_set_invalid(var: *mut QVariant);
    fn qmlrs_variant_set_string(var: *mut QVariant, len: c_uint, data: *const c_char);
    fn qmlrs_variant_get_type(var: *const QVariant) -> QrsVariantType;
    fn qmlrs_variant_get_int(var: *const QVariant, x: *mut c_int);
    fn qmlrs_variant_get_string_length(var: *const QVariant, out: *mut c_uint);
    fn qmlrs_variant_get_string_data(var: *const QVariant, out: *mut c_char);

    fn qmlrs_varlist_create() -> *mut QVariantList;
    fn qmlrs_varlist_destroy(list: *mut QVariantList);
    fn qmlrs_varlist_push(list: *mut QVariantList) -> *mut QVariant;
    fn qmlrs_varlist_length(list: *const QVariantList) -> c_uint;
    fn qmlrs_varlist_get(list: *const QVariantList, i: c_uint) -> *mut QVariant;

    fn qmlrs_app_exec();
}

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
                Variant::Invalid => qmlrs_variant_set_invalid(var),
                Variant::Int(x) => qmlrs_variant_set_int(var, x as c_int),
                Variant::String(ref x) => qmlrs_variant_set_string(var, x.len() as c_uint,
                                                                   x.as_ptr() as *const c_char)
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
                },
                QrsVariantType::String => {
                    let mut len: c_uint = 0;
                    qmlrs_variant_get_string_length(var, &mut len);

                    let mut data: Vec<u8> = Vec::with_capacity(len as uint);
                    qmlrs_variant_get_string_data(var, data.as_mut_ptr() as *mut c_char);
                    data.set_len(len as uint);

                    Variant::String(String::from_utf8_unchecked(data))
                }
            }
        }
    }
}

pub type Slot = Box<FnMut<(Vec<Variant>,),Variant> + 'static>;

struct EngineInternal {
    p: *mut QrsEngine,
    slots: UnsafeCell<HashMap<String, Slot>>
}

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { qmlrs_destroy_engine(self.p); }
    }
}

pub struct Engine {
    nosend: ::std::kinds::marker::NoSend,
    i: Arc<EngineInternal>
}

extern "C" fn slot_fun(slot: *const c_char, data: *mut c_void, result: *mut QVariant,
                       c_args: *mut QVariantList)
{
    /* EngineInternal must be alive here, since the Qml context is alive */

    let i: &EngineInternal = unsafe { std::mem::transmute(data) };
    let cstr = unsafe { CString::new(slot, false) };

    unsafe {
        let mut args = vec![];
        for j in range(0, qmlrs_varlist_length(c_args as *const QVariantList)) {
            let c_arg = qmlrs_varlist_get(c_args as *const QVariantList, j);
            args.push(Variant::get_from(c_arg as *const QVariant));
        }

        /* Must be UTF-8 since these are created from Rust code */
        match (*i.slots.get()).get_mut(cstr.as_str().unwrap()) {
            Some(slot) => slot.call_mut((args,)).set_into(result),
            None       => {
                println!("Warning: unregistered slot called from Qml");
                qmlrs_variant_set_invalid(result);
            }
        }
    }
}

impl Engine {
    pub fn new() -> Engine {
        let p = unsafe { qmlrs_create_engine() };
        assert!(p.is_not_null());

        let i = Arc::new(EngineInternal {
            p: p,
            slots: UnsafeCell::new(HashMap::new())
        });

        unsafe {
            qmlrs_engine_set_slot_function(p, slot_fun, i.deref() as *const EngineInternal
                                                                as *mut c_void);
        }

        Engine {
            nosend: ::std::kinds::marker::NoSend,
            i: i
        }
    }

    pub fn load_url(&mut self, path: &str) {
        unsafe {
            qmlrs_engine_load_url(self.i.p, path.as_ptr() as *const c_char, path.len() as c_uint);
        }
    }

    pub fn register_slot<Sized? S: ToOwned<String>>(&mut self, name: &S, slot: Slot) {
        unsafe {
            (*self.i.slots.get()).insert(name.to_owned(), slot);
        }
    }

    pub fn exec(self) {
        unsafe { qmlrs_app_exec(); }
    }

    pub fn handle(&self) -> Handle {
        Handle { i: self.i.downgrade() }
    }
}

pub struct Handle {
    i: Weak<EngineInternal>
}

impl Handle {
    pub fn invoke(&self, method: &str, args: &[Variant]) -> Result<Variant, &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            let c_args = qmlrs_varlist_create();
            assert!(c_args.is_not_null());
            for arg in args.iter() {
                let c_arg = qmlrs_varlist_push(c_args);
                assert!(c_arg.is_not_null());
                arg.set_into(c_arg);
            }

            let result = qmlrs_variant_create();
            assert!(result.is_not_null());

            match self.i.upgrade() {
                Some(i) => qmlrs_engine_invoke(i.p, cstr.as_ptr(), result,
                                               c_args as *const QVariantList),
                None    => {
                    qmlrs_variant_destroy(result);
                    qmlrs_varlist_destroy(c_args);
                    return Err("View has been freed")
                }
            }

            qmlrs_varlist_destroy(c_args);

            let ret = Variant::get_from(result as *const QVariant);
            qmlrs_variant_destroy(result);

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
