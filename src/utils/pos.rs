use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn move_to(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.y -= 1,
            Dir::South => self.y += 1,
            Dir::West => self.x -= 1,
            Dir::East => self.x += 1,
        }
    }

    pub fn moving_to(mut self, dir: Dir) -> Self {
        self.move_to(dir);
        self
    }

    pub fn is_non_neg(self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Dir::all().map(move |dir| self.moving_to(dir))
    }

    pub fn non_neg_neighbors(self) -> impl Iterator<Item = Self> {
        self.neighbors().filter(|p| p.is_non_neg())
    }

    pub fn diag_neighbors(self) -> impl Iterator<Item = Self> {
        iter!([
            self.moving_to(Dir::West),
            self.moving_to(Dir::East),
            self.moving_to(Dir::North),
            self.moving_to(Dir::South),
            self.moving_to(Dir::North).moving_to(Dir::West),
            self.moving_to(Dir::North).moving_to(Dir::East),
            self.moving_to(Dir::South).moving_to(Dir::West),
            self.moving_to(Dir::South).moving_to(Dir::East),
        ])
    }

    pub fn non_neg_diag_neighbors(self) -> impl Iterator<Item = Self> {
        self.diag_neighbors().filter(|p| p.is_non_neg())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    pub fn all() -> impl Iterator<Item = Self> {
        iter!([Self::North, Self::South, Self::West, Self::East])
    }

    pub fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    pub fn right(self) -> Self {
        self.left().opposite()
    }

    pub fn opposite(self) -> Self {
        self.left().left()
    }

    pub fn turn_left(&mut self) {
        *self = self.left()
    }

    pub fn turn_right(&mut self) {
        *self = self.right()
    }

    pub fn turn_around(&mut self) {
        *self = self.opposite()
    }
}
