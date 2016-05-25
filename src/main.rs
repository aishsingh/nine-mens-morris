extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::TextureQuery;

use std::path::Path;

mod player;
mod piece;

use piece::PlacedPiece;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;
static GAMEAREA_SIZE: u32 = 600;

struct Game {
    turn: u8,
    players: Vec<player::Player>,
    board: Vec<Vec<PlacedPiece>>,

    sdl_context: sdl2::Sdl,
    sdl_renderer: sdl2::render::Renderer<'static>,
    sdl_font: sdl2_ttf::Font,

    selection_active: bool,
    selected_piece: piece::Piece
}

impl Game {
    pub fn run(&mut self) {
        self.board = vec![vec![PlacedPiece::empty;  7]; 7];

        let mut done = false;
        while !done { // Game loop
            self.sdl_renderer.set_draw_color(Color::RGB(0, 0, 0));
            self.sdl_renderer.clear();

            self.draw_board();
            self.draw_ui();
            self.players[0].draw_pieces(&mut self.sdl_renderer);
            self.players[1].draw_pieces(&mut self.sdl_renderer);

            self.sdl_renderer.present();

            done = self.handle_input();
        }
    }

    fn handle_input(&mut self) -> bool {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    return true;
                },

                Event::MouseButtonDown{x, y, ..} => {
                    println!("Mouse Down {} {}", x, y);
                },

                Event::MouseButtonUp{x, y, ..} => {
                    println!("Mouse Up {} {}", x, y);

                    // Add up to 9 pieces
                    if self.players[self.turn as usize -1].total_pieces_count < 9 {
                        let inter: Vec<i32> = self.get_closest_inter(vec![x, y]);
                        if !inter.is_empty() && self.is_inter_avail(vec![inter[0], inter[1]]) { // mouse was near available intersection
                            let p = piece::new(vec![inter[2], inter[3]]);
                            self.players[self.turn as usize -1].add_piece(p);
                            self.board[inter[0] as usize][inter[1] as usize] = PlacedPiece::placed{p: p};
                            self.turn = if self.turn == 2 {1} else {2};
                        }
                    }
                    else if self.selection_active {
                        // TODO
                    }
                },

                _ => {}
            }
        }

        return false;
    }

    fn draw_board(&mut self) {
        let w = GAMEAREA_SIZE as i32;
        let h = GAMEAREA_SIZE as i32;

        let board = [ [vec![(w/8)*1,  h/8],    vec![(w/8)*2,  h/8],    vec![(w/8)*3,  h/8],    vec![(w/8)*4,  h/8],    vec![(w/8)*5,  h/8],    vec![(w/8)*6,  h/8],    vec![(w/8)*7,  h/8]],
                      [vec![(w/8)*1, (h/8)*2], vec![(w/8)*2, (h/8)*2], vec![(w/8)*3, (h/8)*2], vec![(w/8)*4, (h/8)*2], vec![(w/8)*5, (h/8)*2], vec![(w/8)*6, (h/8)*2], vec![(w/8)*7, (h/8)*2]],
                      [vec![(w/8)*1, (h/8)*3], vec![(w/8)*2, (h/8)*3], vec![(w/8)*3, (h/8)*3], vec![(w/8)*4, (h/8)*3], vec![(w/8)*5, (h/8)*3], vec![(w/8)*6, (h/8)*3], vec![(w/8)*7, (h/8)*3]],
                      [vec![(w/8)*1, (h/8)*4], vec![(w/8)*2, (h/8)*4], vec![(w/8)*3, (h/8)*4], vec![(w/8)*4, (h/8)*4], vec![(w/8)*5, (h/8)*4], vec![(w/8)*6, (h/8)*4], vec![(w/8)*7, (h/8)*4]],
                      [vec![(w/8)*1, (h/8)*5], vec![(w/8)*2, (h/8)*5], vec![(w/8)*3, (h/8)*5], vec![(w/8)*4, (h/8)*5], vec![(w/8)*5, (h/8)*5], vec![(w/8)*6, (h/8)*5], vec![(w/8)*7, (h/8)*5]],
                      [vec![(w/8)*1, (h/8)*6], vec![(w/8)*2, (h/8)*6], vec![(w/8)*3, (h/8)*6], vec![(w/8)*4, (h/8)*6], vec![(w/8)*5, (h/8)*6], vec![(w/8)*6, (h/8)*6], vec![(w/8)*7, (h/8)*6]],
                      [vec![(w/8)*1, (h/8)*7], vec![(w/8)*2, (h/8)*7], vec![(w/8)*3, (h/8)*7], vec![(w/8)*4, (h/8)*7], vec![(w/8)*5, (h/8)*7], vec![(w/8)*6, (h/8)*7], vec![(w/8)*7, (h/8)*7]] ];

        self.sdl_renderer.set_draw_color(Color::RGB(150, 150, 150));

        // draw intersections
        for i in 0..7 {
            for j in 0..7 {
                // Skip empty intersections
                if ((i == 0 || i == 6) && ((j > 0 && j < 3) || (j > 3 && j < 6))) ||
                   ((j == 0 || j == 6) && ((i > 0 && i < 3) || (i > 3 && i < 6))) ||
                   ((i == 1 || i == 5) && ((j == 2) || (j == 4))) ||
                   ((j == 1 || j == 5) && ((i == 2) || (i == 4))) ||
                   ((i == 3) && j == 3) {
                    continue;
                }

                self.sdl_renderer.fill_rect(Rect::new(board[i][j][0], board[i][j][1], 5, 5));
            }
        }

        // Intersections
        self.sdl_renderer.draw_line(Point::new(2+board[0][3][0], 2+board[0][3][1]), Point::new(2+board[2][3][0], 2+board[2][3][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[4][3][0], 2+board[4][3][1]), Point::new(2+board[6][3][0], 2+board[6][3][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[3][0][0], 2+board[3][0][1]), Point::new(2+board[3][2][0], 2+board[3][2][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[3][4][0], 2+board[3][4][1]), Point::new(2+board[3][6][0], 2+board[3][6][1]));

        // Outer border
        self.sdl_renderer.draw_line(Point::new(2+board[0][0][0], 2+board[0][0][1]), Point::new(2+board[0][6][0], 2+board[0][6][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[6][0][0], 2+board[6][0][1]), Point::new(2+board[6][6][0], 2+board[6][6][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[0][0][0], 2+board[0][0][1]), Point::new(2+board[6][0][0], 2+board[6][0][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[0][6][0], 2+board[0][6][1]), Point::new(2+board[6][6][0], 2+board[6][6][1]));
                                                                                                          
        // Mid border                                                                                     
        self.sdl_renderer.draw_line(Point::new(2+board[1][1][0], 2+board[1][1][1]), Point::new(2+board[1][5][0], 2+board[1][5][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[5][1][0], 2+board[5][1][1]), Point::new(2+board[5][5][0], 2+board[5][5][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[1][1][0], 2+board[1][1][1]), Point::new(2+board[5][1][0], 2+board[5][1][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[1][5][0], 2+board[1][5][1]), Point::new(2+board[5][5][0], 2+board[5][5][1]));
                                                                                                          
        // Inner border                                                                                   
        self.sdl_renderer.draw_line(Point::new(2+board[2][2][0], 2+board[2][2][1]), Point::new(2+board[2][4][0], 2+board[2][4][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[4][2][0], 2+board[4][2][1]), Point::new(2+board[4][4][0], 2+board[4][4][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[2][2][0], 2+board[2][2][1]), Point::new(2+board[4][2][0], 2+board[4][2][1]));
        self.sdl_renderer.draw_line(Point::new(2+board[2][4][0], 2+board[2][4][1]), Point::new(2+board[4][4][0], 2+board[4][4][1]));
    }

    fn draw_ui(&mut self) {
        // Draw turn into
        let surface = self.sdl_font.render(&format!("{}{}{}", "Player ", self.turn, "'s turn")).blended(self.players[self.turn as usize -1].colour).unwrap();
        let mut texture = self.sdl_renderer.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(SCREEN_WIDTH as i32 - width as i32 - 10, 10, width, height);

        self.sdl_renderer.copy(&mut texture, None, Some(target));
    }

    fn get_closest_inter(&self, pos: Vec<i32>) -> Vec<i32> {
        let w = GAMEAREA_SIZE as i32;
        let h = GAMEAREA_SIZE as i32;

        let board = [ [vec![(w/8)*1,  h/8],    vec![(w/8)*2,  h/8],    vec![(w/8)*3,  h/8],    vec![(w/8)*4,  h/8],    vec![(w/8)*5,  h/8],    vec![(w/8)*6,  h/8],    vec![(w/8)*7,  h/8]],
                      [vec![(w/8)*1, (h/8)*2], vec![(w/8)*2, (h/8)*2], vec![(w/8)*3, (h/8)*2], vec![(w/8)*4, (h/8)*2], vec![(w/8)*5, (h/8)*2], vec![(w/8)*6, (h/8)*2], vec![(w/8)*7, (h/8)*2]],
                      [vec![(w/8)*1, (h/8)*3], vec![(w/8)*2, (h/8)*3], vec![(w/8)*3, (h/8)*3], vec![(w/8)*4, (h/8)*3], vec![(w/8)*5, (h/8)*3], vec![(w/8)*6, (h/8)*3], vec![(w/8)*7, (h/8)*3]],
                      [vec![(w/8)*1, (h/8)*4], vec![(w/8)*2, (h/8)*4], vec![(w/8)*3, (h/8)*4], vec![(w/8)*4, (h/8)*4], vec![(w/8)*5, (h/8)*4], vec![(w/8)*6, (h/8)*4], vec![(w/8)*7, (h/8)*4]],
                      [vec![(w/8)*1, (h/8)*5], vec![(w/8)*2, (h/8)*5], vec![(w/8)*3, (h/8)*5], vec![(w/8)*4, (h/8)*5], vec![(w/8)*5, (h/8)*5], vec![(w/8)*6, (h/8)*5], vec![(w/8)*7, (h/8)*5]],
                      [vec![(w/8)*1, (h/8)*6], vec![(w/8)*2, (h/8)*6], vec![(w/8)*3, (h/8)*6], vec![(w/8)*4, (h/8)*6], vec![(w/8)*5, (h/8)*6], vec![(w/8)*6, (h/8)*6], vec![(w/8)*7, (h/8)*6]],
                      [vec![(w/8)*1, (h/8)*7], vec![(w/8)*2, (h/8)*7], vec![(w/8)*3, (h/8)*7], vec![(w/8)*4, (h/8)*7], vec![(w/8)*5, (h/8)*7], vec![(w/8)*6, (h/8)*7], vec![(w/8)*7, (h/8)*7]] ];

        let offset = 25;
        for i in 0..7 {
            for j in 0..7 {
                // Skip empty intersections
                if ((i == 0 || i == 6) && ((j > 0 && j < 3) || (j > 3 && j < 6))) ||
                   ((j == 0 || j == 6) && ((i > 0 && i < 3) || (i > 3 && i < 6))) ||
                   ((i == 1 || i == 5) && ((j == 2) || (j == 4))) ||
                   ((j == 1 || j == 5) && ((i == 2) || (i == 4))) ||
                   ((i == 3) && j == 3) {
                    continue;
                }

                if (pos[0] - board[i][j][0]).abs() <= offset && (pos[1] - board[i][j][1]).abs() <= offset {
                    return vec![i as i32, j as i32, board[i][j][0], board[i][j][1]];
                }
            }
        }

        // Not close enought to any intersection
        return vec![];
    }

    fn is_inter_avail(&self, inter: Vec<i32>) -> bool {
        match self.board[inter[0] as usize][inter[1] as usize] {
            PlacedPiece::empty => true,
            _ => false,
        }
    }
}

fn main() {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let ttf_context = sdl2_ttf::init().unwrap();

    let window = video_subsystem.window("Nine Men's Morris", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    // Load a font
    let mut font = ttf_context.load_font(Path::new("font/Arvo-Regular.ttf"), 24).unwrap();
    // font.set_style(sdl2_ttf::STYLE_BOLD);

    let mut g = Game {turn: 1, 
        players: vec![player::new(Color::RGB(225, 50, 50)), player::new(Color::RGB(55, 50, 255))], 
        board: vec![],
        sdl_context: context, 
        sdl_renderer: renderer, 
        sdl_font: font,
        selection_active: false,
        selected_piece: piece::new(vec![0, 0])};

    g.run();
}
