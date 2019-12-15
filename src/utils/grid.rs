use super::Pos;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    ops::{Index, IndexMut},
};

pub struct Grid<T> {
    default: T,
    elements: HashMap<Pos, T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(default: T) -> Self {
        Self { default, elements: HashMap::new() }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, pos: Pos) -> &Self::Output {
        self.elements.get(&pos).unwrap_or(&self.default)
    }
}

impl<T: Clone> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let x = &self.default;
        self.elements.entry(pos).or_insert_with(|| x.clone())
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (xs, ys) = match (try {
            let min_x = self.elements.keys().map(|p| p.x).min()?;
            let max_x = self.elements.keys().map(|p| p.x).max()?;
            let min_y = self.elements.keys().map(|p| p.y).min()?;
            let max_y = self.elements.keys().map(|p| p.y).max()?;
            (min_x - 1..=max_x + 1, min_y - 1..=max_y + 1)
        }) {
            Some(x) => x,
            None => return write!(f, "an empty grid"),
        };

        writeln!(
            f,
            "x from {} to {}, y from {} to {}",
            xs.start(),
            xs.end(),
            ys.start(),
            ys.end()
        )?;

        for y in ys {
            for x in xs.clone() {
                write!(f, "{:?}", self[Pos { x, y }])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
