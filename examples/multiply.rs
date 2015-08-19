#[macro_use]
extern crate qmlrs;


struct Multiply;
impl Multiply {
    fn calculate(&self, x: i64, y: i64) -> i64 {
        x * y
    }
}

Q_OBJECT! { Multiply:
    slot fn calculate(i64, i64);
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("multiply", Multiply);
    engine.load_local_file("examples/multiply_ui.qml");

    engine.exec();
}
