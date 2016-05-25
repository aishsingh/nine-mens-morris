extern crate sdl2;

use piece;
use action;
use action::Action;

pub struct Player {
    pub colour: sdl2::pixels::Color,
    pieces: Vec<piece::Piece>,
    pub total_pieces_count: usize,

    pub actions: Action
}

impl Player {
    // Accessors
    pub fn pieces_count(&self) -> usize { self.pieces.len() }

    pub fn draw_pieces(&self, renderer: &mut sdl2::render::Renderer) {
        for i in 0..self.pieces_count() {
            self.pieces[i as usize].draw_piece(renderer);
        }
    }

    pub fn add_piece(&mut self, p: piece::Piece) {
        self.pieces.push(p);
        self.total_pieces_count += 1;
    }

    pub fn remove_piece(&self) {

    }
}

pub fn new(colour: sdl2::pixels::Color) -> Player { Player { colour: colour, pieces: vec![], total_pieces_count: 0, actions: action::new() } }
