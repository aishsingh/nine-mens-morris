use std::collections::LinkedList;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Player {
    pub pieces_count : u8
}

pub trait Piece_Actions {
    fn draw_pieces(&self, renderer: &mut sdl2::render::Renderer);
    fn add_piece(&self);
    fn remove_piece(&self);
}

impl Piece_Actions for Player {
    fn draw_pieces(&self, renderer: &mut sdl2::render::Renderer) {
        for i in 1..self.pieces_count {
            renderer.set_draw_color(Color::RGB(100, 100, 100));
            renderer.fill_rect(Rect::new(100*(i as i32), 100, 50, 50));
        }
    }

    fn add_piece(&self) {

    }

    fn remove_piece(&self) {

    }
}

pub fn new(pieces: u8) -> Player { Player{pieces_count: pieces} }
