extern crate libc;

use libc::{c_char, c_int, c_uint, c_void};
use std::sync::Arc;
use ffi::{QVariant, QrsEngine, QObject};
use std::path::Path;
use std::convert::AsRef;

/* Re-exports */

pub use variant::{Variant, FromQVariant, ToQVariant};
pub use ffi::QVariant as OpaqueQVariant;

/* Submodules */

#[allow(dead_code)]
mod ffi;
mod macros;
mod variant;

pub trait Object {
    fn qt_metaobject(&self) -> MetaObject;
    fn qt_metacall(&mut self, slot: i32, args: *const *const OpaqueQVariant);
}

pub fn __qobject_emit<T: Object>(obj: &T, id: u32) {
    unsafe {
        ffi::qmlrs_object_emit_signal(get_qobject(obj), id as c_uint);
    }
}

/* This is unsafe in case the user copies the data. Might need to figure out something different. */
fn get_qobject<T: Object>(ptr: &T) -> *mut QObject {
    unsafe {
        let t_addr: usize = std::mem::transmute(ptr);
        let hdr: &PropHdr<T> = std::mem::transmute(t_addr - std::mem::size_of::<*mut QObject>());
        hdr.qobj
    }
}

struct EngineInternal {
    p: *mut QrsEngine,
}

/* Hack to get invoke working. Need to figure out better way for invokes anyway.. */
unsafe impl Send for EngineInternal { }
unsafe impl Sync for EngineInternal { }

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { ffi::qmlrs_destroy_engine(self.p); }
    }
}

pub struct Engine {
    i: Arc<EngineInternal>,
}

#[repr(packed)]
struct PropHdr<T: Object> {
    qobj: *mut QObject,
    obj: T
}

extern "C" fn slot_handler<T: Object>(data: *mut c_void, slot: c_int,
                                      args: *const *const ffi::QVariant)
{
    unsafe {
        let hdr: &mut PropHdr<T> = std::mem::transmute(data);
        hdr.obj.qt_metacall(slot as i32, args);
    }
}

impl Engine {
    pub fn new() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine() };
        assert!(!p.is_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            i: i
        }
    }

    pub fn new_headless() -> Engine {
        let p = unsafe { ffi::qmlrs_create_engine_headless() };
        assert!(!p.is_null());

        let i = Arc::new(EngineInternal {
            p: p,
        });

        Engine {
            i: i
        }
    }

    pub fn load_url(&mut self, path: &str) {
        unsafe {
            ffi::qmlrs_engine_load_url(self.i.p, path.as_ptr() as *const c_char,
                                       path.len() as c_uint);
        }
    }

    pub fn load_data(&mut self, data: &str) {
        unsafe {
            ffi::qmlrs_engine_load_from_data(self.i.p, data.as_ptr() as *const c_char,
                                             data.len() as c_uint);
        }
    }



    pub fn load_local_file<P: AsRef<Path>>(&mut self, name: P) {
        let path_raw = std::env::current_dir().unwrap().join(name);
        let path
            = if cfg!(windows) {
                format!("file:///{}",path_raw.display())
            } else {
                format!("file://{}",path_raw.display())
            } ;
        self.load_url(&path);
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
            let mut boxed = Box::new(PropHdr { qobj: std::ptr::null_mut(), obj: obj });
            let qobj = ffi::qmlrs_metaobject_instantiate(
                mo, slot_handler::<T>, &mut *boxed as *mut PropHdr<T> as *mut c_void);

            boxed.qobj = qobj;

            ffi::qmlrs_engine_set_property(self.i.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, qobj);

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
        assert!(!p.is_null());

        MetaObject { p: p }
    }

    pub fn slot(self, name: &str, argc: u8) -> MetaObject {
        unsafe {
            ffi::qmlrs_metaobject_add_slot(self.p, name.as_ptr() as *const c_char,
                                           name.len() as c_uint, argc as c_uint);
        }
        self
    }

    pub fn signal(self, name: &str, argc: u8) -> MetaObject {
        unsafe {
            ffi::qmlrs_metaobject_add_signal(self.p, name.as_ptr() as *const c_char,
                                             name.len() as c_uint, argc as c_uint);
        }
        self
    }
}

/*
pub struct Handle {
    i: Weak<EngineInternal>
}

impl Handle {
    pub fn invoke(&self, method: &str, args: &[Variant]) -> Result<Option<Variant>, &'static str> {
        unsafe {
            let cstr = method.to_c_str();

            let c_args = ffi::qmlrs_varlist_create();
            assert!(!c_args.is_null());
            for arg in args.iter() {
                let c_arg = ffi::qmlrs_varlist_push(c_args);
                assert!(!c_arg.is_null());
                arg.to_qvariant(c_arg);
            }

            let result = ffi::qmlrs_variant_create();
            assert!(!result.is_null());

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
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_engine() {
        Engine::new_headless();
    }
}
