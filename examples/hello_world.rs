extern crate qmlrs;

#[allow(unused_must_use)]
fn main() {
    let mut view = qmlrs::View::new();

    view.set_source("file:///home/cyndis/src/qmlrs/examples/hello.qml");
    view.show();

    let handle = view.handle();
    view.register_slot("hello".into_string(), box move || {
        handle.invoke("hello");
    });

    view.exec();
}
