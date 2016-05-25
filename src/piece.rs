extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy,Clone)]
pub struct Piece {
    x: i32,
    y: i32
}

impl Piece {
    pub fn draw_piece(&self, renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(Color::RGB(225, 50, 50));
        renderer.fill_rect(Rect::new(self.x, self.y, 30, 30));
    }
}

pub fn new(pos: Vec<i32>) -> Piece { Piece { x: pos[0], y: pos[1] } }
