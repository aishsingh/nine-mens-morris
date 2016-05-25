extern crate sdl2;

use sdl2::pixels::Color;

use piece;
use piece::PlacedPiece;

pub struct Action {
    result: bool,
    // history: Vec<>
}

pub trait PlaceAction {
    fn place_piece(&self, p: &piece::Piece, board: &mut Vec<Vec<PlacedPiece>>, inter: &Vec<i32>) -> bool;
}

pub trait MoveAction {
    fn move_piece(&self, p: &piece::Piece) -> bool;
}

pub trait JumpAction {
    fn jump_piece(&self, p: &piece::Piece) -> bool;
}

impl PlaceAction for Action {
    fn place_piece(&self, p: &piece::Piece, board: &mut Vec<Vec<PlacedPiece>>, inter: &Vec<i32>) -> bool {
        if self.is_inter_avail(vec![inter[0], inter[1]], &board) {
            board[inter[0] as usize][inter[1] as usize] = PlacedPiece::placed{p: *p};

            // detect mills
            if self.is_mill(&p, vec![inter[0], inter[1]], &board) {
                println!("Mill created!");
            }

            true
        }
        else {
            false
        }
    }
}

impl MoveAction for Action {
    fn move_piece(&self, p: &piece::Piece) -> bool {
        false
    }
}
impl JumpAction for Action {
    fn jump_piece(&self, p: &piece::Piece) -> bool {
        false
    }
}

impl Action {
    fn is_inter_avail(&self, inter: Vec<i32>, board: &Vec<Vec<PlacedPiece>>) -> bool {
        match board[inter[0] as usize][inter[1] as usize] {
            PlacedPiece::empty => true,
            _ => false,
        }
    }

    fn is_friendly_piece(&self, p: &piece::Piece, inter: Vec<i32>, board: &Vec<Vec<PlacedPiece>>) -> bool {
        // if !self.is_inter_avail(vec![inter[0], inter[1]], &board) {
        //     if board[inter[0] as usize][inter[1] as usize].
        // }

        let col = match board[inter[0] as usize][inter[1] as usize] {
            PlacedPiece::placed{p} => {p.colour},
            _ => (Color::RGB(0, 0, 0))
        };

        if p.colour == col {
            true
        }
        else {
            false
        }

    }

    fn is_mill(&self, p: &piece::Piece, inter: Vec<i32>, board: &Vec<Vec<PlacedPiece>>) -> bool {
        // let outer_border = if (inter[0] == 0 || inter[0] == 6) || (inter[1] == 0 || inter[1] == 6) {true} else {false};

        if inter[0] == 0 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 0], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 0], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 0], &board) {
                if self.is_friendly_piece(&p, vec![3, 0], &board) {
                    return true;
                }
            }
        }



        if inter[0] == 1 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 1], &board) {
                if self.is_friendly_piece(&p, vec![5, 1], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 1], &board) {
                if self.is_friendly_piece(&p, vec![5, 1], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 0], &board) {
                if self.is_friendly_piece(&p, vec![3, 0], &board) {
                    return true;
                }
            }
        }


        if inter[0] == 2 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 2], &board) {
                if self.is_friendly_piece(&p, vec![4, 2], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 2], &board) {
                if self.is_friendly_piece(&p, vec![4, 2], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 4 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 2], &board) {
                if self.is_friendly_piece(&p, vec![3, 2], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 0 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 3], &board) {
                if self.is_friendly_piece(&p, vec![2, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 1 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 3], &board) {
                if self.is_friendly_piece(&p, vec![2, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 2 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 3], &board) {
                if self.is_friendly_piece(&p, vec![1, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 4 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![5, 3], &board) {
                if self.is_friendly_piece(&p, vec![6, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 3], &board) {
                if self.is_friendly_piece(&p, vec![6, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 3], &board) {
                if self.is_friendly_piece(&p, vec![5, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 4 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![5, 3], &board) {
                if self.is_friendly_piece(&p, vec![6, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 3], &board) {
                if self.is_friendly_piece(&p, vec![6, 3], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 3], &board) {
                if self.is_friendly_piece(&p, vec![5, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 2 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 4], &board) {
                if self.is_friendly_piece(&p, vec![4, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 4], &board) {
                if self.is_friendly_piece(&p, vec![4, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 4 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 4], &board) {
                if self.is_friendly_piece(&p, vec![3, 4], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 1 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 5], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 5], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 5], &board) {
                if self.is_friendly_piece(&p, vec![3, 5], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 1 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 5], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 5], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 5], &board) {
                if self.is_friendly_piece(&p, vec![3, 5], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 0 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 6], &board) {
                if self.is_friendly_piece(&p, vec![6, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 6], &board) {
                if self.is_friendly_piece(&p, vec![6, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 6], &board) {
                if self.is_friendly_piece(&p, vec![3, 6], &board) {
                    return true;
                }
            }
        }

        // Horizontal axis

        if inter[0] == 0 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 0], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 0], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![0, 0], &board) {
                if self.is_friendly_piece(&p, vec![3, 0], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 1 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 3], &board) {
                if self.is_friendly_piece(&p, vec![1, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 1 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 1], &board) {
                if self.is_friendly_piece(&p, vec![1, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 1 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![1, 1], &board) {
                if self.is_friendly_piece(&p, vec![1, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 2 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 3], &board) {
                if self.is_friendly_piece(&p, vec![2, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 2 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 2], &board) {
                if self.is_friendly_piece(&p, vec![2, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 2 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![2, 2], &board) {
                if self.is_friendly_piece(&p, vec![2, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 3 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 1], &board) {
                if self.is_friendly_piece(&p, vec![3, 2], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 0], &board) {
                if self.is_friendly_piece(&p, vec![3, 2], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 0], &board) {
                if self.is_friendly_piece(&p, vec![3, 1], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 3 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 5], &board) {
                if self.is_friendly_piece(&p, vec![3, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 4], &board) {
                if self.is_friendly_piece(&p, vec![3, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 3 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![3, 4], &board) {
                if self.is_friendly_piece(&p, vec![3, 5], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 4 && inter[1] == 2 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 3], &board) {
                if self.is_friendly_piece(&p, vec![4, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 4 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 2], &board) {
                if self.is_friendly_piece(&p, vec![4, 4], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 4 && inter[1] == 4 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![4, 2], &board) {
                if self.is_friendly_piece(&p, vec![4, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 5 && inter[1] == 1 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![5, 3], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![5, 1], &board) {
                if self.is_friendly_piece(&p, vec![5, 5], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 5 && inter[1] == 5 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![5, 1], &board) {
                if self.is_friendly_piece(&p, vec![5, 3], &board) {
                    return true;
                }
            }
        }

        if inter[0] == 6 && inter[1] == 0 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![6, 3], &board) {
                if self.is_friendly_piece(&p, vec![6, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 3 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![6, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 6], &board) {
                    return true;
                }
            }
        }
        if inter[0] == 6 && inter[1] == 6 {
            // Check neighbours
            if self.is_friendly_piece(&p, vec![6, 0], &board) {
                if self.is_friendly_piece(&p, vec![6, 3], &board) {
                    return true;
                }
            }
        }

        false
    }
}

pub fn new() -> Action { Action { result: false } }
