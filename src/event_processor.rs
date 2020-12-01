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
        Event::RedrawRequested(_) => set_background(&(1.0, 0.85, 0.38), &gl, &gl_context),
        _ => *control_flow = ControlFlow::Wait
    }
}

fn set_background(rgb: &(f32, f32, f32), gl: &Gl, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
    let gl = gl.deref();
    unsafe {
        gl.ClearColor(rgb.0, rgb.1, rgb.2, 1.0);
        gl.Clear(gl_bindings::COLOR_BUFFER_BIT);
    }
    gl_context.swap_buffers().expect("Error while swapping buffers.");
}
