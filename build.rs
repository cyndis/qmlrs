extern crate pkg_config;

use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;
use std::path::PathBuf;

fn main() {
    let wcd = env::current_dir().unwrap();
    let build = PathBuf::from(&wcd.join("ext/libqmlrswrapper/build"));

    let _ = fs::create_dir_all(&build);

    let mut myargs = vec![".."] ;


    /*
     * Support Qt installed via the Ports system on BSD-like systems.
     *
     * The native libs are in `/usr/local/lib`, which is not linked against by default.
     * This means that either the user or every package has to add this if they want to link
     * against something that is not part of the core distribution in `/usr/lib`.
     *
     * See https://wiki.freebsd.org/WarnerLosh/UsrLocal for the line of reasoning & how this will
     * change in the future.
     */
    if cfg!(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd",
                target_os = "dragonfly", target_os = "bitrig")) {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }


    /*
     * Prameters for supporting QT on OS X installed via homebres
     *
     * Because QT5 conflicts with QT4 the homebrew package manager won't link
     * the QT5 package into the default search paths for libraries, to deal
     * with this we need to give pkg-config and cmake a nudge in the right
     * direction.
     */
    if cfg!(target_os = "macos") {
        // Point at homebrew's QT5 install location, this will likely fail if
        // another package manager was used.
        let qt5_lib_path = Path::new("/usr/local/opt/qt5/lib");

        if Path::exists(qt5_lib_path) {
            // First nudge cmake in the direction of the .cmake files added by
            // homebrew. This clobbers the existing value if present, it's
            // unlikely to be present though.
            env::set_var("CMAKE_PREFIX_PATH", qt5_lib_path.join("cmake"));

            // Nudge pkg-config in the direction of the brewed QT to ensure the
            // correct compiler flags get found for the project.
            env::set_var("PKG_CONFIG_PATH", qt5_lib_path.join("pkgconfig"));
        } else {
            panic!("The QT5 was not found at the expected location ({}) please install it via homebrew.", qt5_lib_path.display());
        }
    }

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
