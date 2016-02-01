#[macro_use]
extern crate qmlrs;

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
    let engine = qmlrs::Engine::new();

    engine.set_property("factorial", Factorial);
    engine.load_local_file("examples/factorial_ui.qml");

    engine.exec();
}
