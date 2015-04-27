extern crate pkg_config;

use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));

    let _ = fs::create_dir_all(&build);

    let mut myargs = vec![".."] ;
    let is_msys = env::var("MSYSTEM").is_ok() ;
    if cfg!(windows) && is_msys {
        myargs.push("-GMSYS Makefiles") ;
    }
    Command::new("cmake").args(&myargs).current_dir(&build).output().unwrap_or_else(|e| {
        panic!("Failed to run cmake: {}", e);
    });

    Command::new("make").current_dir(&build).output().unwrap_or_else(|e| {
        panic!("Failed to run make: {}", e);
    });

    if cfg!(windows) && is_msys {
        println!("cargo:rustc-link-search=native={}\\system32",env::var("WINDIR").unwrap());
    }
    println!("cargo:rustc-link-lib=static=qmlrswrapper");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search=native={}",build.display());
    pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick").unwrap();
}
