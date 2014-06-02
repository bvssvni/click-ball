use graphics::*;
use piston::*;
use std::rand::random;

use {
    App,
    Ball,
    Pos,
    Window,
};

fn pick_random(w: f64, h: f64, margin: f64) -> Pos {
    Pos {
        x: (w - margin * 2.0) * random() + margin,
        y: (h - margin * 2.0) * random() + margin,
    }
}

pub fn run(app: &mut App, game_iter: &mut GameIterator<Window>) {
    let random_margin = 20.0;
    
    let mut ball = Ball {
        radius: 10.0,
        pos: Pos { x: 100.0, y: 100.0 }
    };

    loop {
        match game_iter.next() { None => { break }, Some(e) => match e {
            Render(args) => {
                app.render_update(&args);
                
                // Draw ball.
                let c = Context::abs(args.width as f64, args.height as f64);
                ball.draw(&c, args.gl);
            },
            MouseMove(args) => {
                app.mouse_move_update(&args);
            },
            MousePress(_args) => {
                let d = app.mouse - ball.pos;
                let inside = d.len() <= ball.radius;
                if inside {
                    ball.pos = pick_random(
                        app.screen_width as f64, 
                        app.screen_height as f64,
                        random_margin);
                }
            },
            _ => {},
        } }
    }
}
