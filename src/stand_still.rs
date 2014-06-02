use {
    Pos,
};

pub struct Behavior {
    pub pos: Pos,
}

impl Behavior {
    pub fn create_motion(&self) -> Motion {
        Motion {
            pos: self.pos,
        }
    }
}

pub struct Motion {
    pub pos: Pos,
}

impl Motion {
    pub fn next(&self, dt: f64) -> Option<Motion> {
        Some(Motion {
            pos: self.pos
        })
    }
}

