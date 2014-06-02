#![feature(globs)]

extern crate piston;
extern crate graphics;

use graphics::*;
use piston::*;
use std::rand::random;

pub use Pos = pos::Pos;
pub use Ball = ball::Ball;

mod pos;
mod ball;

pub struct Line {
    a: Pos,
    b: Pos,
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn pick_random(w: f64, h: f64, margin: f64) -> Pos {
    Pos {
        x: (w - margin * 2.0) * random() + margin,
        y: (h - margin * 2.0) * random() + margin,
    }
}

fn main() {
    let mut screen_width = 300;
    let mut screen_height = 300;
    
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Click-Ball".to_string(),
            size: [screen_width, screen_height],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut ball = Ball {
        radius: 10.0,
        pos: Pos { x: 100.0, y: 100.0 }
    };

    let mut mouse_pos = Pos { x: 0.0, y: 0.0 };

    let random_margin = 20.0;
    
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    loop {
        match game_iter.next() { None => { break }, Some(e) => match e {
            Render(args) => {
                // Update screen width and height.
                screen_width = args.width;
                screen_height = args.height;
                
                // Draw ball.
                let c = Context::abs(args.width as f64, args.height as f64);
                ball.draw(&c, args.gl);
            },
            MouseMove(args) => {
                mouse_pos = Pos { x: args.x, y: args.y };
            },
            MousePress(_args) => {
                let d = mouse_pos - ball.pos;
                let inside = d.len() <= ball.radius;
                if inside {
                    ball.pos = pick_random(
                        screen_width as f64, 
                        screen_height as f64,
                        random_margin);
                }
            },
            _ => {},
        } }
    }
}

