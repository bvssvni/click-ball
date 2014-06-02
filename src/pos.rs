
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
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
