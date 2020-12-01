extern crate glutin;

use glutin::{
    ContextWrapper,
    event::{ElementState, VirtualKeyCode},
    event_loop::ControlFlow,
    PossiblyCurrent,
    window::{Fullscreen, Window},
};

pub fn process_key(
    gl_context: &ContextWrapper<PossiblyCurrent, Window>,
    input: (VirtualKeyCode, ElementState),
    control_flow: &mut ControlFlow,
) {
    match input {
        (VirtualKeyCode::Escape, _) => {
            *control_flow = ControlFlow::Exit
        }
        (VirtualKeyCode::F, ElementState::Pressed) => toggle_fullscreen(gl_context),
        _ => (),
    }
}

fn toggle_fullscreen(gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
    let window = gl_context.window();
    if let None = window.fullscreen() {
        let monitor = window.primary_monitor().expect("Primary monitor not found.");
        let fullscreen = Fullscreen::Borderless(Some(monitor));
        window.set_fullscreen(Some(fullscreen))
    } else {
        window.set_fullscreen(None)
    }
}
