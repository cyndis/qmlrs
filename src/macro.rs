#[macro_export]
macro_rules! Q_OBJECT(
    (
        $t:ty :
            $(
                slot fn $name:ident ( ) ;
            )*
    ) => (
        impl qmlrs::Object for $t {
            fn qt_metaobject(&self) -> qmlrs::MetaObject {
                let x = qmlrs::MetaObject::new();
                $(
                    let x = x.method(stringify!($name), 0);
                )+
                x
            }

            #[allow(unused_assignments)]
            fn qt_metacall(&mut self, slot: i32) {
                let mut i = 0;
                $(
                    if i == slot {
                        self.$name();
                        return
                    }
                    i += 1;
                )+
            }
        }
    )
)
