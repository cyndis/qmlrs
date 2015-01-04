extern crate "pkg-config" as pkg_config;

use std::io::Command;
use std::os;

#[allow(unused_must_use)]
fn main() {
    let mut build = os::getcwd().unwrap();
    build.push_many(&["ext", "libqmlrswrapper", "build"]);

    /* Ignore error, the return value is not reliable and we'll catch it when chdir'ing anyway. */
    std::io::fs::mkdir(&build, std::io::USER_RWX);

    os::change_dir(&build).ok().expect("Failed to change into build directory");

    let out = Command::new("cmake").arg("..").output();
    if out.unwrap().status != std::io::process::ProcessExit::ExitStatus(0) {
        panic!("Failed to run cmake");
    }

    let out = Command::new("make").output();
    if out.unwrap().status != std::io::process::ProcessExit::ExitStatus(0) {
        panic!("Failed to run make");
    }

    println!("cargo:rustc-flags=-L {} -l qmlrswrapper:static -l stdc++", build.display());
    pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick").unwrap();
}
