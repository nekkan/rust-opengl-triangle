extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut file_gl = File::create(&Path::new(&out_dir)
        .join("gl_bindings.rs"))
        .unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 6),
        Profile::Core,
        Fallbacks::All, [
            "GL_NV_command_list",
        ],
    );

    registry.write_bindings(
        StructGenerator,
        &mut file_gl,
    ).unwrap();
}
