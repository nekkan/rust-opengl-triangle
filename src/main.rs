extern crate bindings;
extern crate glutin;

use glutin::{
    ContextBuilder,
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::Gl;

fn main() {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title("Simple OpenGL Game")
        .with_inner_size(LogicalSize::new(900.0, 400.0));

    let gl_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .expect("Couldn't create the window context.");

    let gl_context = unsafe { gl_context.make_current().unwrap() };
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => gl_context.resize(size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                redraw();
                gl_context.swap_buffers().expect("Error while swapping buffers.");
            }
            _ => *control_flow = ControlFlow::Wait
        }
    });
}

fn redraw() {
    // TODO
}
