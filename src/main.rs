extern crate bindings;
extern crate glutin;

use std::{io::{stdin, stdout, Write}, ops::Deref};
use std::borrow::Borrow;

use glutin::{
    ContextBuilder,
    ContextWrapper,
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::{MonitorHandle, VideoMode},
    PossiblyCurrent,
    window::{Fullscreen, Window, WindowBuilder},
};

use bindings::{Gl, gl_bindings};

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


    event_loop.run(move |event, _, control_flow| {
        process_event(&gl_context, &gl, event, control_flow);
    });
}

fn process_event(
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
                process_key(gl_context, (virtual_key_code, state), control_flow)
            }
            _ => ()
        },
        Event::RedrawRequested(_) => {
            redraw(&(255.0 / 255.0, 219.0 / 255.0, 98.0 / 255.0), &gl, &gl_context)
        }
        _ => *control_flow = ControlFlow::Wait
    }
}

fn redraw(rgb: &(f32, f32, f32), gl: &Gl, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
    let gl = gl.deref();
    unsafe {
        gl.ClearColor(rgb.0, rgb.1, rgb.2, 1.0);
        gl.Clear(gl_bindings::COLOR_BUFFER_BIT);
    }
    gl_context.swap_buffers().expect("Error while swapping buffers.");
}

fn process_key(
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

/*fn prompt_for_video_mode(monitor: &MonitorHandle) -> VideoMode {
    for (i, video_mode) in monitor.video_modes().enumerate() {
        println!("Video mode #{}: {}", i, video_mode);
    }

    print!("Please write the number of the video mode to use: ");
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    let user_input = user_input
        .trim()
        .parse()
        .ok()
        .expect("Invalid number.");

    let video_mode = monitor
        .video_modes()
        .nth(user_input)
        .expect("Invalid ID.");

    println!("Using video mode {}", video_mode);

    video_mode
}
*/
