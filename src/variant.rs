use libc::{c_char, c_int, c_uint};
use ffi;
use ffi::{QVariant, QrsVariantType};

pub enum Variant {
    Int(int),
    String(String),
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
            Variant::Int(ref x) => x.to_qvariant(var),
            Variant::String(ref s) => s.to_qvariant(var),
        }
    }
}
