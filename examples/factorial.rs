#[macro_use]
extern crate qmlrs;

use std::fs::File;
use std::io::prelude::*;

struct Factorial;
impl Factorial {
    fn calculate(&self, x: i64) -> i64 {
        (1..x+1).fold(1, |t,c| t * c)
    }
}

Q_OBJECT! { Factorial:
    slot fn calculate(i64);
//    signal fn test();
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", Factorial);
    engine.load_local_file("examples/factorial_ui.qml");

    engine.exec();

    // test with string:
    let mut engine2 = qmlrs::Engine::new();
    engine2.set_property("factorial", Factorial);
    let mut qml_file = File::open("examples/factorial_ui.qml").unwrap();
    let mut qml_string = String::new();
    qml_file.read_to_string(&mut qml_string).unwrap();
    qml_string = qml_string.replace("Factorial", "Factorial (from string)");
    engine2.load_data(&qml_string);

    engine2.exec();
}
