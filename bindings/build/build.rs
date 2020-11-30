extern crate gl_generator;
extern crate gl_generator_profiling_struct;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use gl_generator_profiling_struct::ProfilingStructGenerator;

mod profiling_struct_generator;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 6),
        Profile::Core,
        Fallbacks::All, [
            "GL_NV_command_list",
        ],
    );

    registry.write_bindings(
        ProfilingStructGenerator,
        &mut file_gl,
    ).unwrap();
}
