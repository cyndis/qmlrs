extern crate qmlrs;

#[allow(unused_must_use)]
fn main() {
    let mut view = qmlrs::Engine::new();

    view.load_url("file:///home/cyndis/src/qmlrs/examples/hello.qml");

    let handle = view.handle();
    view.register_slot("hello", box move |args| {
        println!("Rust-side hello called with: {}", args);
        let foo = handle.invoke("hello", &[qmlrs::Variant::Int(555)]).unwrap();
        println!("QML hello slot returned: {}", foo);
        qmlrs::Variant::Int(42)
    });

    view.exec();
}
