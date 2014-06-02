//! Move in straight line.

use {
    Pos,
};

pub struct Behavior {
    pub start: Pos,
    pub end: Pos,
    pub vel: f64,
}

impl Behavior {
    pub fn create_motion(&self) -> Motion {
        Motion {
            pos: self.start,
            target: self.end,
            vel: self.vel,
        }
    }
}

pub struct Motion {
    pub pos: Pos,
    pub vel: f64,
    pub target: Pos,
}

impl Motion {
    pub fn next(&self, dt: f64) -> Option<Motion> {
        let dx = self.target.x - self.pos.x;
        let dy = self.target.y - self.pos.y;
        let d = (dx * dx + dy * dy).sqrt();
        if d < self.vel * dt {
            return None;
        }
        let dx = dx / d;
        let dy = dy / d;
        Some(Motion {
            pos: Pos {
                x: self.pos.x + dt * dx,
                y: self.pos.y + dt * dy
            },
            ..*self
        })
    }
}


