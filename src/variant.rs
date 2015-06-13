use libc::{c_char, c_uint};
use ffi;
use ffi::{QVariant, QrsVariantType};

pub enum Variant {
    I64(i64),
    Bool(bool),
    String(String),
}

pub trait FromQVariant {
    fn from_qvariant(arg: *const QVariant) -> Option<Self>;
}

impl FromQVariant for i64 {
    fn from_qvariant(var: *const QVariant) -> Option<i64> {
        unsafe {
            if ffi::qmlrs_variant_get_type(var) == QrsVariantType::Int64 {
                let mut x: i64 = 0;
                ffi::qmlrs_variant_get_int64(var, &mut x);
                Some(x)
            } else {
                None
            }
        }
    }
}

impl FromQVariant for bool {
    fn from_qvariant(var: *const QVariant) -> Option<bool> {
        unsafe {
            if ffi::qmlrs_variant_get_type(var) == QrsVariantType::Bool {
                let mut x: bool = false;
                ffi::qmlrs_variant_get_bool(var, &mut x);
                Some(x)
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

                let mut data: Vec<u8> = Vec::with_capacity(len as usize);
                ffi::qmlrs_variant_get_string_data(var, data.as_mut_ptr() as *mut c_char);
                data.set_len(len as usize);

                Some(String::from_utf8_unchecked(data))
            } else {
                None
            }
        }
    }
}

impl FromQVariant for Variant {
    fn from_qvariant(var: *const QVariant) -> Option<Variant> {
        use ffi::QrsVariantType::*;
        unsafe {
            match ffi::qmlrs_variant_get_type(var) {
                Int64 =>
                    Some(Variant::I64(FromQVariant::from_qvariant(var).unwrap())),
                Bool =>
                    Some(Variant::Bool(FromQVariant::from_qvariant(var).unwrap())),
                String =>
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

impl ToQVariant for i64 {
    fn to_qvariant(&self, var: *mut QVariant) {
        unsafe {
            ffi::qmlrs_variant_set_int64(var, *self);
        }
    }
}

impl ToQVariant for bool {
    fn to_qvariant(&self, var: *mut QVariant) {
        unsafe {
            ffi::qmlrs_variant_set_bool(var, *self);
        }
    }
}

macro_rules! int_toqvar {
    ($($t:ty)*) => (
        $(
        impl ToQVariant for $t {
            fn to_qvariant(&self, var: *mut QVariant) {
                (*self as i64).to_qvariant(var);
            }
        }
        )*
    )
}

int_toqvar!(u8 u16 u32 i8 i16 i32 isize);

impl ToQVariant for str {
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
            Variant::I64(ref x) => x.to_qvariant(var),
            Variant::Bool(ref x) => x.to_qvariant(var),
            Variant::String(ref s) => s.to_qvariant(var),
        }
    }
}
