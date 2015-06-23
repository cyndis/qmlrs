#![allow(missing_copy_implementations)]

use libc::{c_char, c_int, c_uint, c_void};

pub enum QrsEngine {}
pub enum QrsMetaObject {}
pub enum QObject {}
pub enum QVariant {}
pub enum QVariantList {}

#[repr(C)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum QrsVariantType {
    Invalid = 0,
    Int64,
    Bool,
    String
}

pub type SlotFunction = extern "C" fn(data: *mut c_void, id: c_int, args: *const *const QVariant);

extern "C" {
    pub fn qmlrs_create_engine() -> *mut QrsEngine;
    pub fn qmlrs_create_engine_headless() -> *mut QrsEngine;
    pub fn qmlrs_destroy_engine(engine: *mut QrsEngine);
    pub fn qmlrs_engine_load_url(engine: *mut QrsEngine, path: *const c_char, len: c_uint);
    pub fn qmlrs_engine_load_from_data(engine: *mut QrsEngine, data: *const c_char, len: c_uint);
    pub fn qmlrs_engine_invoke(engine: *mut QrsEngine, method: *const c_char, result: *mut QVariant,
                               args: *const QVariantList);
    pub fn qmlrs_engine_set_property(engine: *mut QrsEngine, name: *const c_char, len: c_uint,
                                     object: *mut QObject);

    pub fn qmlrs_variant_create() -> *mut QVariant;
    pub fn qmlrs_variant_destroy(v: *mut QVariant);
    pub fn qmlrs_variant_set_int64(var: *mut QVariant, x: i64);
    pub fn qmlrs_variant_set_bool(var: *mut QVariant, x: bool);
    pub fn qmlrs_variant_set_invalid(var: *mut QVariant);
    pub fn qmlrs_variant_set_string(var: *mut QVariant, len: c_uint, data: *const c_char);
    pub fn qmlrs_variant_get_type(var: *const QVariant) -> QrsVariantType;
    pub fn qmlrs_variant_get_int64(var: *const QVariant, x: *mut i64);
    pub fn qmlrs_variant_get_bool(var: *const QVariant, x: *mut bool);
    pub fn qmlrs_variant_get_string_length(var: *const QVariant, out: *mut c_uint);
    pub fn qmlrs_variant_get_string_data(var: *const QVariant, out: *mut c_char);

    pub fn qmlrs_varlist_create() -> *mut QVariantList;
    pub fn qmlrs_varlist_destroy(list: *mut QVariantList);
    pub fn qmlrs_varlist_push(list: *mut QVariantList) -> *mut QVariant;
    pub fn qmlrs_varlist_length(list: *const QVariantList) -> c_uint;
    pub fn qmlrs_varlist_get(list: *const QVariantList, i: c_uint) -> *mut QVariant;

    pub fn qmlrs_app_exec();

    pub fn qmlrs_metaobject_create() -> *mut QrsMetaObject;
    pub fn qmlrs_metaobject_destroy(mo: *mut QrsMetaObject);
    pub fn qmlrs_metaobject_add_slot(mo: *mut QrsMetaObject, name: *const c_char, name_len: c_uint,
                                     argc: c_uint);
    pub fn qmlrs_metaobject_add_signal(mo: *mut QrsMetaObject, name: *const c_char,
                                       name_len: c_uint, argc: c_uint);
    pub fn qmlrs_metaobject_instantiate(mo: *mut QrsMetaObject, fun: SlotFunction,
                                        data: *mut c_void) -> *mut QObject;

    pub fn qmlrs_object_emit_signal(obj: *mut QObject, id: c_uint);
    pub fn qmlrs_object_destroy(obj: *mut QObject);
}
