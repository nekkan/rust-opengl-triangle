use std::ops::Deref;
use std::rc::Rc;

pub use crate::bindings::*;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F: FnMut(&'static str) -> *const types::GLvoid>(loadfn: F) -> Gl {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn)),
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}

