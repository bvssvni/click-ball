
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn zero() -> Pos {
        Pos {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Sub<Pos, Pos> for Pos {
    fn sub(&self, rhs: &Pos) -> Pos {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


