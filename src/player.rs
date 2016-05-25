extern crate sdl2;

use piece;

pub struct Player {
    pieces_count: usize,
    pieces: Vec<piece::Piece>
}

impl Player {
    pub fn draw_pieces(&self, renderer: &mut sdl2::render::Renderer) {
        for i in 0..self.pieces_count {
            self.pieces[i as usize].draw_piece(renderer);
        }
    }

    pub fn add_piece(&self) {

    }

    pub fn remove_piece(&self) {

    }
}

pub fn new(pieces_count: usize) -> Player { Player { pieces_count: pieces_count, pieces: vec![piece::new(vec![0, 0]); pieces_count] } }
