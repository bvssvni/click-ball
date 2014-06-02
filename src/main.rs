#![feature(globs)]

extern crate piston;
extern crate graphics;

use piston::*;

pub use Pos = pos::Pos;
pub use Ball = ball::Ball;
pub use App = app::App;

mod pos;
mod ball;
mod app;
mod level_1;

type Window = GameWindowSDL2;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let screen_width = 300;
    let screen_height = 300;
    
    let mut window: Window = GameWindow::new(
        GameWindowSettings {
            title: "Click-Ball".to_string(),
            size: [screen_width, screen_height],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut app = App {
        screen_width: screen_width,
        screen_height: screen_height,
        mouse: Pos::zero(),
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);

    level_1::run(&mut app, &mut game_iter);    
}

