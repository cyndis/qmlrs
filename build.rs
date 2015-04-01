#![feature(convert)]

extern crate pkg_config;

use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));

    let _ = fs::create_dir_all(&build);

    Command::new("cmake").arg("..").current_dir(&build).output().unwrap_or_else(|e| {
        panic!("Failed to run cmake: {}", e);
    });

    Command::new("make").current_dir(&build).output().unwrap_or_else(|e| {
        panic!("Failed to run make: {}", e);
    });

    println!("cargo:rustc-flags=-L {} -l qmlrswrapper:static -l stdc++", build.display());
    pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick").unwrap();
}
