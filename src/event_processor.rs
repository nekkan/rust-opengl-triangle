extern crate bindings;
extern crate glutin;

use std::ops::Deref;

use glutin::{
    ContextWrapper,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
    PossiblyCurrent,
    window::Window,
};

use bindings::{Gl, gl_bindings};

use crate::key_processor;

pub fn process_event(
    vao: u32,
    shader_id: u32,
    gl_context: &ContextWrapper<PossiblyCurrent, Window>,
    gl: &Gl,
    event: Event<()>,
    control_flow: &mut ControlFlow,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => gl_context.resize(size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    virtual_keycode: Some(virtual_key_code),
                    state,
                    ..
                },
                ..
            } => {
                key_processor::process_key(gl_context, (virtual_key_code, state), control_flow)
            }
            _ => ()
        },
        Event::RedrawRequested(_) => {
            set_background(&(1.0, 0.85, 0.38), &gl);
            let deref_gl = gl.deref();
            unsafe {
                deref_gl.UseProgram(shader_id);
                deref_gl.BindVertexArray(vao);
                deref_gl.DrawArrays(gl_bindings::TRIANGLES, 0, 3);
            }
            gl_context.swap_buffers().expect("Error while swapping buffers.");
        }
        _ => *control_flow = ControlFlow::Wait
    }
}

fn set_background(rgb: &(f32, f32, f32), gl: &Gl) {
    let gl = gl.deref();
    unsafe {
        gl.ClearColor(rgb.0, rgb.1, rgb.2, 1.0);
        gl.Clear(gl_bindings::COLOR_BUFFER_BIT);
    }
}
