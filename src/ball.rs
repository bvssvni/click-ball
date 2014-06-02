use graphics::*;
use piston::*;

use Pos;

pub struct Ball {
    pub pos: Pos,
    pub radius: f64,
}

impl Ball {
    pub fn draw(&self, c: &Context, gl: &mut Gl) {
        c
        .circle_centered(self.pos.x, self.pos.y, self.radius)
        .rgb(1.0, 0.0, 0.0)
        .fill(gl);
    }
}

