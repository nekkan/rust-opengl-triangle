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
        .with_title("Rust OpenGL Triangle")
        .with_inner_size(LogicalSize::new(900.0, 600.0));

    let event_loop = EventLoop::new();

    let gl_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .expect("Couldn't create the window context.");

    let gl_context = unsafe { gl_context.make_current().unwrap() };
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let deref_gl = gl.deref();
    let shader = create_shader(deref_gl);
    let vao = triangle::get_triangle_vao(deref_gl);

    event_loop.run(move |event, _, control_flow| {
        event_processor::process_event(vao, shader.id, &gl_context, &gl, event, control_flow);
    });
}

fn create_shader(gl: &gl_bindings::Gl) -> Shader {
    let vertex_code = include_str!("../shaders/triangle.vert");
    let fragment_code = include_str!("../shaders/triangle.frag");

    Shader::new(gl, vertex_code, fragment_code)
}
