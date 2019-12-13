use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn left(mut self) -> Self {
        self.x -= 1;
        self
    }

    pub fn right(mut self) -> Self {
        self.x += 1;
        self
    }

    pub fn up(mut self) -> Self {
        self.y -= 1;
        self
    }

    pub fn down(mut self) -> Self {
        self.y += 1;
        self
    }

    pub fn is_non_neg(self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        iter!([self.left(), self.right(), self.up(), self.down()])
    }

    pub fn non_neg_neighbors(self) -> impl Iterator<Item = Self> {
        self.neighbors().filter(|p| p.is_non_neg())
    }

    pub fn diag_neighbors(self) -> impl Iterator<Item = Self> {
        iter!([
            self.left(),
            self.right(),
            self.up(),
            self.down(),
            self.up().left(),
            self.up().right(),
            self.down().left(),
            self.down().right()
        ])
    }

    pub fn non_neg_diag_neighbors(self) -> impl Iterator<Item = Self> {
        self.diag_neighbors().filter(|p| p.is_non_neg())
    }
}
