#![feature(unboxed_closures, globs, macro_rules, unsafe_destructor)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void};
use std::sync::{Arc, Weak};

use ffi::{QVariant, QrsVariantType, QrsEngine, QVariantList};
pub use ffi::QVariant as OpaqueQVariant;
pub mod ffi;
mod macro;

pub enum Variant {
    Int(int),
    String(String)
}

pub trait FromQVariant {
    fn from_qvariant(arg: *const QVariant) -> Option<Self>;
}

impl FromQVariant for int {
    fn from_qvariant(var: *const QVariant) -> Option<int> {
        unsafe {
            if ffi::qmlrs_variant_get_type(var) == QrsVariantType::Int {
                let mut x: c_int = 0;
                ffi::qmlrs_variant_get_int(var, &mut x);
                Some(x as int)
            } else {
                None
            }
        }
    }
}

impl FromQVariant for String {
    fn from_qvariant(var: *const QVariant) -> Option<String> {
        unsafe {
            if ffi::qmlrs_variant_get_type(var) == QrsVariantType::String {
                let mut len: c_uint = 0;
                ffi::qmlrs_variant_get_string_length(var, &mut len);

                let mut data: Vec<u8> = Vec::with_capacity(len as uint);
                ffi::qmlrs_variant_get_string_data(var, data.as_mut_ptr() as *mut c_char);
                data.set_len(len as uint);

                Some(String::from_utf8_unchecked(data))
            } else {
                None
            }
        }
    }
}

impl FromQVariant for Variant {
    fn from_qvariant(var: *const QVariant) -> Option<Variant> {
        unsafe {
            match ffi::qmlrs_variant_get_type(var) {
                QrsVariantType::Int =>
                    Some(Variant::Int(FromQVariant::from_qvariant(var).unwrap())),
                QrsVariantType::String =>
                    Some(Variant::String(FromQVariant::from_qvariant(var).unwrap())),
                _ => None
            }
        }
    }
}

pub trait ToQVariant {
    fn to_qvariant(&self, var: *mut QVariant);
}

impl ToQVariant for () {
    fn to_qvariant(&self, var: *mut QVariant) {
        unsafe {
            ffi::qmlrs_variant_set_invalid(var);
        }
    }
}

impl ToQVariant for int {
    fn to_qvariant(&self, var: *mut QVariant) {
        unsafe {
            ffi::qmlrs_variant_set_int(var, *self as c_int);
        }
    }
}

impl ToQVariant for String {
    fn to_qvariant(&self, var: *mut QVariant) {
        unsafe {
            ffi::qmlrs_variant_set_string(var, self.len() as c_uint,
                                          self.as_ptr() as *const c_char);
        }
    }
}

impl ToQVariant for Variant {
    fn to_qvariant(&self, var: *mut QVariant) {
        match *self {
            Variant::Int(ref x) => x.to_qvariant(var),
            Variant::String(ref s) => s.to_qvariant(var),
        }
    }
}

pub trait Object {
    fn qt_metaobject(&self) -> MetaObject;
    fn qt_metacall(&mut self, slot: i32, args: *const *const OpaqueQVariant);
}

struct EngineInternal {
    p: *mut QrsEngine,
}

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { ffi::qmlrs_destroy_engine(self.p); }
    }
}

struct HeldProp {
    p: *mut (),
    qp: *mut ffi::QObject,
    ty: *const std::intrinsics::TyDesc
}

pub struct Engine {
    nosend: ::std::kinds::marker::NoSend,
    i: Arc<EngineInternal>,
    held: Vec<HeldProp>
}

#[unsafe_destructor]
impl Drop for Engine {
    fn drop(&mut self) {
        for held in self.held.iter() {
            unsafe {
                ((*held.ty).drop_glue)(std::mem::transmute(&held.p));
                ffi::qmlrs_object_destroy(held.qp);
            }
        }
    }
}

extern "C" fn slot_handler<T: Object>(data: *mut c_void, slot: c_int,
                                      args: *const *const ffi::QVariant)
{
    unsafe {
        let obj: &mut T = std::mem::transmute(data);
        obj.qt_metacall(slot as i32, args);
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
            i: i,
            held: vec![]
        }
    }

    pub fn new_headless() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine_headless() };
        assert!(p.is_not_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            nosend: ::std::kinds::marker::NoSend,
            i: i,
            held: vec![]
        }
    }

    pub fn load_url(&mut self, path: &str) {
        unsafe {
            ffi::qmlrs_engine_load_url(self.i.p, path.as_ptr() as *const c_char,
                                       path.len() as c_uint);
        }
    }

    pub fn exec(self) {
        unsafe { ffi::qmlrs_app_exec(); }
    }

    /*
    pub fn handle(&self) -> Handle {
        Handle { i: self.i.downgrade() }
    }
    */

    pub fn set_property<T: Object>(&mut self, name: &str, obj: T) {
        unsafe {
            let mo = obj.qt_metaobject().p;
            let mut boxed = box obj;
            let qobj = ffi::qmlrs_metaobject_instantiate(mo, slot_handler::<T>,
                                                         &mut *boxed as *mut T as *mut c_void);

            ffi::qmlrs_engine_set_property(self.i.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, qobj);

            /* Uhh.. */
            self.held.push(HeldProp { p: &mut *boxed as *mut T as *mut (), qp: qobj,
                                      ty: std::intrinsics::get_tydesc::<Box<T>>() });

            std::mem::forget(boxed);
        }
    }
}

/* MetaObjects currently leak. Once a cache system is implemented, this should be fine. */

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
    pub fn invoke(&self, method: &str, args: &[Variant]) -> Result<Option<Variant>, &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            let c_args = ffi::qmlrs_varlist_create();
            assert!(c_args.is_not_null());
            for arg in args.iter() {
                let c_arg = ffi::qmlrs_varlist_push(c_args);
                assert!(c_arg.is_not_null());
                arg.to_qvariant(c_arg);
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

            let ret = FromQVariant::from_qvariant(result as *const QVariant);
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
        Engine::new_headless();
    }
}
