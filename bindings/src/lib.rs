use std::ops::Deref;
use std::rc::Rc;

pub use crate::gl_bindings::*;

pub mod shader;

pub mod gl_bindings {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

#[derive(Clone)]
pub struct Gl {
    pub gl: Rc<gl_bindings::Gl>,
}

impl Gl {
    pub fn load_with<F: FnMut(&'static str) -> *const types::GLvoid>(loadfn: F) -> Gl {
        Gl {
            gl: Rc::new(gl_bindings::Gl::load_with(loadfn)),
        }
    }
}

impl Deref for Gl {
    type Target = gl_bindings::Gl;

    fn deref(&self) -> &gl_bindings::Gl {
        &self.gl
    }
}

