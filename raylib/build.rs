use std::env;
use std::path::PathBuf;

fn main() {
    // Need to generate the .lib file for Raylib:
    // 1. gendef raylib.dll
    // 2. dlltool -d raylib.def -l raylib.lib
    println!("cargo:rustc-link-search=native=C:\\raylib-5.0_win64_mingw-w64\\lib");
    println!("cargo:rustc-link-lib=static=raylib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
