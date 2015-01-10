# qmlrs - QtQuick bindings for Rust

![Image of example](https://raw.githubusercontent.com/cyndis/qmlrs/ghstatic/screenshot.png)

qmlrs allows the use of Qml/QtQuick code from Rust, specifically

- Rust code can create a QtQuick engine (QQmlApplicationEngine) with a loaded Qml script
- Qml code can invoke Rust functions
- Qml code can connect to signals defined in Rust

..with certain limitations. The library should be safe (as in not `unsafe`) to use, but no promises
at this time. Reviews of the code would be welcome.

## Requirements

The library consists of a Rust part and a C++ part. The C++ part will be compiled automatically
when building with Cargo. You will need `cmake`, Qt5 and a C++ compiler that can compile Qt5 code.
Your Qt5 installation should have at least the following modules: Core, Gui, Qml, Quick and Quick Controls.

## Current limitations

- The Engine holds ownership of all properties and signal emission requires a reference to one.
  This means that signals can currently only be emitted from slot handlers making them not very
  useful.

## Example

This is the Rust code for an application allowing the calculation of factorials.
(Also contains a test for signals.)
You can find the corresponding Qml code in the `examples` directory.

```rust
#![allow(unstable)]

#[macro_use]
extern crate qmlrs;

struct Factorial;
impl Factorial {
    fn calculate(&self, x: i64) -> i64 {
        std::iter::range_inclusive(1, x).fold(1, |t,c| t * c)
    }
}

Q_OBJECT! { Factorial:
    slot fn calculate(i64);
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", Factorial);
    engine.load_local_file(&Path::new("examples/factorial_ui.qml"));

    engine.exec();
}

```

## Note regarding the Qt event loop and threads

Creating an `Engine` automatically initializes the Qt main event loop if one doesn't already exist.
At least on some operating systems, the event loop must run on the main thread. Qt will tell you
if you mess up. The `.exec()` method on views starts the event loop. This will block the thread
until the window is closed.

Qt objects have a thread affinity, and their methods must be called on the thread they were created
on. For this reason, `Engine`s are `NoSend`. However, you can create sendable handles using the `.handle()`
method (the name "handle" probably should change). Handles have built-in logic to allow invoking
Qml methods from other threads, but they will not keep the `Engine` alive. Method calls will return
an error if the underlying `Engine` has been destroyed.

## Licensing

The code in this library is dual-licensed under the MIT license and the Apache License (version 2.0).
See LICENSE-APACHE and LICENSE-MIT for details.
