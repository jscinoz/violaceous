#![feature(plugin)]
#![plugin(clippy)]

extern crate bindgen;
extern crate pkg_config;

use std::path::{Path};
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("purple.rs");
    let mut bindings = bindgen::builder();

    let purple_lib = pkg_config::probe_library("purple").unwrap();

    let purple_header = purple_lib.include_paths[0].join("purple.h");
    let purple_header = purple_header.to_str().unwrap();

    println!("libpurple main header: {}", purple_header);
    // Set main header path
    bindings.header(purple_header);

    for include_path in purple_lib.include_paths {
        let include_path = include_path.to_str().unwrap();

        println!("Adding include dir: {}", include_path);

        // Add each required include dir provided by pkg-config
        bindings.clang_arg("-I").clang_arg(include_path);
    }

    bindings.forbid_unknown_types();
//    bindings.emit_builtins();
    // Only generate bindings for things under libpurple. Otherwise we end up
    // generating bindings for glib too
    // XXX: Can't currently do this. Tried linking with glib-sys, but couldn't end up getting it
    // to build
    //bindings.match_pat("libpurple");

    // XXX: Do we need to do this, or is the `links` entry in Cargo.toml sufficient?
    // bindings.link("purple");

    // Don't derive debug - causes output bindings not to be compilable
    bindings.derive_debug(false);

    let bindings = bindings.generate();
    
    match bindings {
        Ok(bindings) => {
            bindings.write_to_file(out_file).unwrap();
        }
        _ => panic!("Bindings generation failed")
    }
}
