extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use gl_generator_profiling_struct::ProfilingStructGenerator;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("Generating gl_bindings.rs file...");

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 6),
        Profile::Core,
        Fallbacks::All,
        ["GL_NV_command_list"]
    );

    registry
        .write_bindings(ProfilingStructGenerator, &mut file)
        .unwrap();
}
