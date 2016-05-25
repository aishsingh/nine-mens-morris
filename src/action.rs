extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Action {
    result: bool;
    // history: Vec<>
}

struct 

trait MoveAction {
    pub fn move_piece();
}

trait JumpAction {
    pub fn jump_piece();
}

impl MoveAction for Action {
    pub fn move_piece() {

    }
}
impl JumpAction for Action {
    pub fn jump_piece() {

    }
}

pub fn new() -> Action { Action { result: false } }
