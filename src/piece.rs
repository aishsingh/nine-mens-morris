extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy,Clone)]
pub struct Piece {
    x: i32,
    y: i32,
    pub colour: sdl2::pixels::Color
}

#[derive(Copy,Clone)]
pub enum PlacedPiece {
    placed { p: Piece },
    empty
}

impl Piece {
    pub fn draw_piece(&self, renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(self.colour);
        renderer.fill_rect(Rect::new(self.x - (30/2), self.y - (30/2), 30, 30));
    }
}

pub fn new(pos: Vec<i32>, colour: sdl2::pixels::Color) -> Piece { Piece { x: pos[0], y: pos[1], colour: colour } }
