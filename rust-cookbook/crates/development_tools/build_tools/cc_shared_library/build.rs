use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build `libmylibrary.so` the way you would outside of a Rust project:
    // with its own Makefile, independent of the `cc` crate.
    let status = Command::new("make")
        .current_dir(&manifest_dir)
        .env("OUT_DIR", &out_dir)
        .status()
        .expect("failed to run `make`; is a C++ toolchain installed?");
    assert!(status.success(), "failed to build libmylibrary.so");

    // Tell rustc where to find `libmylibrary` at compile time...
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    // ...and bake a runtime search path into the binary, so the dynamically
    // linked library can be found without setting `LD_LIBRARY_PATH` by hand.
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=mylibrary");

    println!("cargo:rerun-if-changed=src/mylibrary.cc");
    println!("cargo:rerun-if-changed=src/mylibrary.h");
    println!("cargo:rerun-if-changed=Makefile");
}
