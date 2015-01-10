#![allow(unstable)]

extern crate qmlrs;

#[test]
fn test_quit() {
    let mut engine = qmlrs::Engine::new_headless();

    let mut path = std::os::getcwd().unwrap();
    path.push_many(&["tests", "simple_test.qml"]);
    path = std::os::make_absolute(&path).unwrap();

    engine.load_url(format!("file://{}", path.display()).as_slice());

    engine.exec();
}
