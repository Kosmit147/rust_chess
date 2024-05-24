use self::{chess_board::ChessBoard, renderer::Renderer};

mod chess_board;
mod renderer;

pub struct App {
    board: ChessBoard,
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        App {
            board: ChessBoard::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn render(&self, frame: &mut glium::Frame) {
        self.renderer.render(frame);
    }
}
