#![feature(phase)]

#[phase(plugin, link)]
extern crate qmlrs;

struct Factorial;
impl Factorial {
    fn calculate(&self, x: int) -> int {
        std::iter::range_inclusive(1, x).fold(1, |t,c| t * c)
    }
}

Q_OBJECT!( Factorial:
    slot fn calculate(int);
);

fn main() {
    let mut engine = qmlrs::Engine::new();

    let mut path = std::os::getcwd().unwrap();
    path.push_many(&["examples", "factorial_ui.qml"]);
    path = std::os::make_absolute(&path).unwrap();

    engine.load_local_file(&Path::new("examples/factorial_ui.qml"));

    engine.set_property("factorial", Factorial);

    engine.exec();
}
