#![feature(macro_rules, phase)]

#[phase(plugin, link)]
extern crate qmlrs;

fn factorial(x: int) -> int {
    std::iter::range_inclusive(1, x).fold(1, |t,c| t * c)
}

struct Ses;
impl Ses {
    fn sas(&mut self) {
        println!("Sas tosiaan!");
    }
}

Q_OBJECT!( Ses:
    slot fn sas();
)

fn main() {
    let mut engine = qmlrs::Engine::new();

    let mut path = std::os::getcwd().unwrap();
    path.push_many(&["examples", "factorial_ui.qml"]);
    path = std::os::make_absolute(&path).unwrap();

    engine.load_url(format!("file://{}", path.display()).as_slice());

    engine.set_property("ses", Ses);

    engine.exec();
}
