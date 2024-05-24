use app::App;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

extern crate glium;
extern crate winit;

mod app;

fn main() {
    let app = App::new();

    let event_loop = EventLoop::new().expect("Failed to create winit event loop!");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(800, 400)
        .with_title("Chess")
        .build(&event_loop);

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            elwt.exit();
        }
        Event::AboutToWait => {
            let mut frame = display.draw();
            app.render(&mut frame);
            let _ = frame.finish();
        }
        _ => (),
    });
}
