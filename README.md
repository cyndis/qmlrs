# qmlrs - [QtQuick](http://doc.qt.io/qt-5/qtquick-index.html) bindings for Rust

[![Travis Build Status](https://travis-ci.org/cyndis/qmlrs.svg?branch=master)](https://travis-ci.org/cyndis/qmlrs)
[![Coverage Status](https://coveralls.io/repos/github/cyndis/qmlrs/badge.svg?branch=master)](https://coveralls.io/github/cyndis/qmlrs?branch=master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)
[![crates.io](https://img.shields.io/crates/v/qmlrs.svg)](https://crates.io/crates/qmlrs)

![Image of example](https://raw.githubusercontent.com/cyndis/qmlrs/ghstatic/screenshot.png)

qmlrs allows the use of QML/QtQuick code from Rust, specifically

- Rust code can create a QtQuick engine (QQmlApplicationEngine) with a loaded QML script
- QML code can invoke Rust functions

…with certain limitations. The library should be safe (as in not `unsafe`) to use, but no promises
at this time. Reviews of the code would be welcome.

## News

See the [Changelog](./CHANGELOG.md) for the version history and what's waiting in master to be released.

## Requirements

The library consists of a Rust part and a C++ part. The C++ part will be compiled automatically
when building with Cargo. You will need `cmake`, Qt5 and a C++ compiler that can compile Qt5 code.
Your Qt5 installation should have at least the following modules: Core, Gui, Qml, Quick and Quick Controls.

If you are installing Qt5 from source, please note that passing "-noaccessibility" to the configure
script disables the qtquickcontrols module.

## Usage

Add the latest version of qmlrs from [crates.io](https://crates.io/crates/qmlrs/) in your project's `Cargo.toml`.

## Example

This is the Rust code for an application allowing the calculation of factorials.
You can find the corresponding QML code in the `examples` directory.

```rust
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
    engine.load_local_file("examples/factorial_ui.qml");

    engine.exec();
}

```
To run the above example, execute `cargo run --example factorial` in the project's root directory.

## Note regarding the Qt event loop and threads

Creating an `Engine` automatically initializes the Qt main event loop if one doesn't already exist.
At least on some operating systems, the event loop must run on the main thread. Qt will tell you
if you mess up. The `.exec()` method on views starts the event loop. This will block the thread
until the window is closed.

## Licensing

The code in this library is dual-licensed under the MIT license and the Apache License (version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
