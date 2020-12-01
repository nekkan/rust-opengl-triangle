extern crate bindings;
extern crate glutin;

use std::ops::Deref;

use glutin::{
    ContextBuilder,
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use bindings::{Gl, gl_bindings};
use bindings::shader::Shader;

mod key_processor;
mod event_processor;
mod triangle;

fn main() {
    let window_builder = WindowBuilder::new()
        .with_title("Simple OpenGL Game")
        .with_inner_size(LogicalSize::new(900.0, 400.0));

    let event_loop = EventLoop::new();

    let gl_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .expect("Couldn't create the window context.");

    let gl_context = unsafe { gl_context.make_current().unwrap() };
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    create_shaders(gl.deref());

    event_loop.run(move |event, _, control_flow| {
        event_processor::process_event(&gl_context, &gl, event, control_flow);
    });
}

fn create_shaders(gl: &gl_bindings::Gl) {
    let vertex_code = include_str!("../shaders/triangle.vert");
    let fragment_code = include_str!("../shaders/triangle.frag");

    unsafe {
        let shader = Shader::new(gl, vertex_code, fragment_code);
        gl.UseProgram(shader.id);
    }
}
