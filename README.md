# qmlrs - A QtQuick interface for Rust

qmlrs allows the use of Qml/QtQuick code from Rust, specifically

- Rust code can create a QtQuick window with a loaded Qml script
- Rust code can invoke Qml functions
- Qml code can invoke Rust functions

..with certain limitations. This is currently still proof-of-concept
level code; the interfaces could probably use some improvement. Method parameters
in either direction are also not supported.
The library should be safe (as in not `unsafe`) to use, but no promises.

## Requirements

The library consists of a Rust part and a C++ part. The C++ part will be compiled automatically
when building with Cargo. You will need `cmake`, Qt5 and a C++ compiler that can compile Qt5 code.

## Example

This is the Rust code for an application showing a window with some text that can be changed by
clicking. You can find the corresponding Qml code in the `examples` directory.

```rust
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
```

## Note regarding the Qt event loop and threads

Creating a `View` automatically initializes the Qt main event loop is one doesn't already exist.
At least on some operating systems, the event loop must run on the main thread. Qt will tell you
if you mess up. The `.exec()` method on views starts the event loop. This will block the thread
until the window is closed.

Qt objects have a thread affinity, and their methods must be called on the thread they were created
on. For this reason, `View`s are `NoSend`. However, you can create sendable handles using the `.handle()`
method (the name "handle" probably should change). Handles have built-in logic to allow invoking
Qml methods from other threads, but they will not keep the `View` alive. Method calls will return
an error if the underlying `View` has been destroyed.
