#[macro_export]
macro_rules! Q_OBJECT(
    (
        $t:ty :
            $(
                slot fn $name:ident ( $($at:ty),* );
            )*
    ) => (
        impl qmlrs::Object for $t {
            #[allow(unused_mut, unused_variables)]
            fn qt_metaobject(&self) -> qmlrs::MetaObject {
                let x = qmlrs::MetaObject::new();
                $(
                    let mut argc = 0;
                    $(
                        let _: $at;
                        argc += 1;
                    )*
                    let x = x.method(stringify!($name), argc);
                )+
                x
            }

            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn qt_metacall(&mut self, slot: i32, args: *const *const qmlrs::OpaqueQVariant) {
                use qmlrs::ToQVariant;
                let mut i = 0;
                $(
                    if i == slot {
                        let mut argi = 1u8; /* 0 is for return value */
                        let ret = self.$name(
                            $(
                                {
                                    let _: $at;
                                    match qmlrs::FromQVariant::from_qvariant(unsafe { *args.offset(argi as int) }) {
                                        Some(arg) => { argi += 1; arg }
                                        None      => {
                                            println!("qmlrs: Invalid argument {} type for slot {}", argi, stringify!($name));
                                            return;
                                        }
                                    }
                                }
                            )*
                        );
                        ret.to_qvariant(unsafe { *args.offset(0) as *mut qmlrs::OpaqueQVariant });
                        return
                    }
                    i += 1;
                )+
            }
        }
    )
)
