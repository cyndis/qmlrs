#[macro_use]
extern crate qmlrs;
mod math;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut engine = qmlrs::Engine::new();
    engine.set_property("factorial", math::Factorial);
    let mut qml_file = File::open("examples/factorial_ui.qml").unwrap();
    let mut qml_string = String::new();
    qml_file.read_to_string(&mut qml_string).unwrap();
    qml_string = qml_string.replace("Factorial", "Factorial (from string)");
    engine.load_data(&qml_string);

    engine.exec();
}
