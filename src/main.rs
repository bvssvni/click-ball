#![feature(globs)]

extern crate piston;
extern crate graphics;

use graphics::*;
use piston::*;
use std::rand::random;

mod straight_line;
mod stand_still;

pub struct Pos {
    x: f64,
    y: f64,
}

pub struct Line {
    a: Pos,
    b: Pos,
}

/// A wrapper type for all the kinds of motion.
pub enum Motion {
    StandStillMotion(stand_still::Motion),
    StraightLineMotion(straight_line::Motion),
}

impl Motion {
    fn next(&self, dt: f64) -> Option<Motion> {
        match *self {
            StandStillMotion(move) => match move.next(dt) {
                None => None,
                Some(move) => Some(StandStillMotion(move)),
            },
            StraightLineMotion(move) => match move.next(dt) {
                None => None,
                Some(move) => Some(StraightLineMotion(move)),
            },
        }
    }
}


pub struct StandStillMotion {
    pos: Pos,
}

pub enum Behavior {
    StandStillBehavior(stand_still::Behavior),
    StraightLineBehavior(straight_line::Behavior),
}

impl Behavior {
    fn create_motion(&self) -> Motion {
        match *self {
            StandStillBehavior(behavior) => {
                StandStillMotion(behavior.create_motion())
            },
            StraightLineBehavior(behavior) => {
                StraightLineMotion(behavior.create_motion())
            },
        }    
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut screen_width = 300;
    let mut screen_height = 300;
    
    let levels = [
        StandStillBehavior(stand_still::Behavior {
            pos: Pos { x: 100.0, y: 100.0 }
        }),
        StraightLineBehavior(straight_line::Behavior {
            start: Pos { x: 0.0, y: 100.0 },
            end: Pos { x: screen_width as f64, y: 100.0 },
            vel: 5.0,
        })
    ];

    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Click-Ball".to_string(),
            size: [screen_width, screen_height],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut ball_x = 100.0;
    let mut ball_y = 100.0;
    let ball_radius = 10.0;

    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;

    let random_margin = 20.0;
    let pick_random: |f64, f64| -> (f64, f64) = |w, h| {
        (
            (w - random_margin * 2.0) * random() + random_margin,
            (h - random_margin * 2.0) * random() + random_margin
        )
    };
    
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
                c.circle_centered(ball_x, ball_y, ball_radius).rgb(1.0, 0.0, 0.0).fill(args.gl);
            },
            MouseMove(args) => {
                mouse_x = args.x;
                mouse_y = args.y;
            },
            MousePress(_args) => {
                let (dx, dy) = (mouse_x - ball_x, mouse_y - ball_y);
                let inside = (dx * dx + dy * dy) <= ball_radius * ball_radius;
                if inside {
                    let (rx, ry) = pick_random(screen_width as f64, screen_height as f64);
                    ball_x = rx;
                    ball_y = ry;
                }
            },
            _ => {},
        } }
    }
}

