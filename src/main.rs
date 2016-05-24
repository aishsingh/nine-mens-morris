extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;

mod player;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Nine Men's Morris", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let p = player::new(9);

    let mut done = false;
    while !done { // Game loop
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        draw_board(&mut renderer);

        // p.draw_pieces(&mut renderer);

        renderer.present();

        done = handle_input(&sdl_context);
    }
}

fn handle_input(sdl_context: &sdl2::Sdl) -> bool {
    let mut event_pump = sdl_context.event_pump().unwrap();

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
            },

            _ => {}
        }
    }

    return false;
}

fn draw_board(renderer: &mut sdl2::render::Renderer) {
    let width = 800;
    let height = 800;

    let mut board = [ [vec![(width/8)*1,  height/8],    vec![(width/8)*2,  height/8],    vec![(width/8)*3,  height/8],    vec![(width/8)*4,  height/8],    vec![(width/8)*5,  height/8],    vec![(width/8)*6,  height/8],    vec![(width/8)*7,  height/8]],
                      [vec![(width/8)*1, (height/8)*2], vec![(width/8)*2, (height/8)*2], vec![(width/8)*3, (height/8)*2], vec![(width/8)*4, (height/8)*2], vec![(width/8)*5, (height/8)*2], vec![(width/8)*6, (height/8)*2], vec![(width/8)*7, (height/8)*2]],
                      [vec![(width/8)*1, (height/8)*3], vec![(width/8)*2, (height/8)*3], vec![(width/8)*3, (height/8)*3], vec![(width/8)*4, (height/8)*3], vec![(width/8)*5, (height/8)*3], vec![(width/8)*6, (height/8)*3], vec![(width/8)*7, (height/8)*3]],
                      [vec![(width/8)*1, (height/8)*4], vec![(width/8)*2, (height/8)*4], vec![(width/8)*3, (height/8)*4], vec![(width/8)*4, (height/8)*4], vec![(width/8)*5, (height/8)*4], vec![(width/8)*6, (height/8)*4], vec![(width/8)*7, (height/8)*4]],
                      [vec![(width/8)*1, (height/8)*5], vec![(width/8)*2, (height/8)*5], vec![(width/8)*3, (height/8)*5], vec![(width/8)*4, (height/8)*5], vec![(width/8)*5, (height/8)*5], vec![(width/8)*6, (height/8)*5], vec![(width/8)*7, (height/8)*5]],
                      [vec![(width/8)*1, (height/8)*6], vec![(width/8)*2, (height/8)*6], vec![(width/8)*3, (height/8)*6], vec![(width/8)*4, (height/8)*6], vec![(width/8)*5, (height/8)*6], vec![(width/8)*6, (height/8)*6], vec![(width/8)*7, (height/8)*6]],
                      [vec![(width/8)*1, (height/8)*7], vec![(width/8)*2, (height/8)*7], vec![(width/8)*3, (height/8)*7], vec![(width/8)*4, (height/8)*7], vec![(width/8)*5, (height/8)*7], vec![(width/8)*6, (height/8)*7], vec![(width/8)*7, (height/8)*7]] ];

    // draw intersections
    for i in 0..7 {
        renderer.set_draw_color(Color::RGB(100, 100, 100));
        // renderer.fill_rect(Rect::new((width/4)*(i+1 as i32), (height/8), 5, 5));  // top row
        // renderer.fill_rect(Rect::new((width/8)*(i+1 as i32), (height/8)*2, 5, 5));  // top row
        // renderer.fill_rect(Rect::new((width/8)*(i+1 as i32), ((height/8)*3), 5, 5));  // bottom row
        // renderer.fill_rect(Rect::new((width/8)*(i+1 as i32), ((height/8)*4), 5, 5));  // bottom row
        // renderer.fill_rect(Rect::new((width/8)*(i+1 as i32), ((height/8)*5), 5, 5));  // bottom row
        // renderer.fill_rect(Rect::new((width/8)*(i+1 as i32), ((height/8)*6), 5, 5));  // bottom row
        // renderer.fill_rect(Rect::new((width/4)*(i+1 as i32), ((height/8)*7), 5, 5));  // bottom row
        
        for j in 0..7 {
            // Only draw actual positions
            if ((i == 0 || i == 6) && ((j > 0 && j < 3) || (j > 3 && j < 6))) ||
               ((j == 0 || j == 6) && ((i > 0 && i < 3) || (i > 3 && i < 6))) ||
               ((i == 1 || i == 5) && ((j == 2) || (j == 4))) ||
               ((j == 1 || j == 5) && ((i == 2) || (i == 4))) ||
               ((i == 3) && j == 3) {
                continue;
            }

            renderer.fill_rect(Rect::new(board[i][j][0], board[i][j][1], 5, 5));
        }
    }

    // Intersections
    renderer.draw_line(Point::new(2+board[0][3][0], 2+board[0][3][1]), Point::new(2+board[2][3][0], 2+board[2][3][1]));
    renderer.draw_line(Point::new(2+board[4][3][0], 2+board[4][3][1]), Point::new(2+board[6][3][0], 2+board[6][3][1]));
    renderer.draw_line(Point::new(2+board[3][0][0], 2+board[3][0][1]), Point::new(2+board[3][2][0], 2+board[3][2][1]));
    renderer.draw_line(Point::new(2+board[3][4][0], 2+board[3][4][1]), Point::new(2+board[3][6][0], 2+board[3][6][1]));

    // Outer border
    renderer.draw_line(Point::new(2+board[0][0][0], 2+board[0][0][1]), Point::new(2+board[0][6][0], 2+board[0][6][1]));
    renderer.draw_line(Point::new(2+board[6][0][0], 2+board[6][0][1]), Point::new(2+board[6][6][0], 2+board[6][6][1]));
    renderer.draw_line(Point::new(2+board[0][0][0], 2+board[0][0][1]), Point::new(2+board[6][0][0], 2+board[6][0][1]));
    renderer.draw_line(Point::new(2+board[0][6][0], 2+board[0][6][1]), Point::new(2+board[6][6][0], 2+board[6][6][1]));
                                                                                                      
    // Mid border                                                                                     
    renderer.draw_line(Point::new(2+board[1][1][0], 2+board[1][1][1]), Point::new(2+board[1][5][0], 2+board[1][5][1]));
    renderer.draw_line(Point::new(2+board[5][1][0], 2+board[5][1][1]), Point::new(2+board[5][5][0], 2+board[5][5][1]));
    renderer.draw_line(Point::new(2+board[1][1][0], 2+board[1][1][1]), Point::new(2+board[5][1][0], 2+board[5][1][1]));
    renderer.draw_line(Point::new(2+board[1][5][0], 2+board[1][5][1]), Point::new(2+board[5][5][0], 2+board[5][5][1]));
                                                                                                      
    // Inner border                                                                                   
    renderer.draw_line(Point::new(2+board[2][2][0], 2+board[2][2][1]), Point::new(2+board[2][4][0], 2+board[2][4][1]));
    renderer.draw_line(Point::new(2+board[4][2][0], 2+board[4][2][1]), Point::new(2+board[4][4][0], 2+board[4][4][1]));
    renderer.draw_line(Point::new(2+board[2][2][0], 2+board[2][2][1]), Point::new(2+board[4][2][0], 2+board[4][2][1]));
    renderer.draw_line(Point::new(2+board[2][4][0], 2+board[2][4][1]), Point::new(2+board[4][4][0], 2+board[4][4][1]));
}
