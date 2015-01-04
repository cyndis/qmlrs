#![feature(phase)]

#[phase(plugin, link)]
extern crate qmlrs;

struct Factorial;
impl Factorial {
    fn calculate(&self, x: int) -> int {
        self.test();
        std::iter::range_inclusive(1, x).fold(1, |t,c| t * c)
    }
}

Q_OBJECT! { Factorial:
    slot fn calculate(int);
    signal fn test();
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", Factorial);
    engine.load_local_file(&Path::new("examples/factorial_ui.qml"));

    engine.exec();
}
