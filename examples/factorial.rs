extern crate qmlrs;

fn factorial(x: int) -> int {
    std::iter::range_inclusive(1, x).fold(1, |t,c| t * c)
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    let mut path = std::os::getcwd().unwrap();
    path.push_many(&["examples", "factorial_ui.qml"]);
    path = std::os::make_absolute(&path).unwrap();

    engine.load_url(format!("file://{}", path.display()).as_slice());

    engine.register_slot("calculate", box move |args| {
        if let qmlrs::Variant::Int(x) = args[0] {
            qmlrs::Variant::Int(factorial(x))
        } else {
            qmlrs::Variant::Invalid
        }
    });

    engine.exec();
}
