use glium::Surface;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn render(&self, frame: &mut glium::Frame) {
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
    }
}
