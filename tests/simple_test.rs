extern crate qmlrs;

#[test]
fn test_quit() {
    let mut engine = qmlrs::Engine::new_headless();

    engine.load_local_file("tests/simple_test.qml");

    engine.exec();
}
