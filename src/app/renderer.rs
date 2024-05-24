use glium::Surface;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, frame: &mut glium::Frame) {
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
    }
}
