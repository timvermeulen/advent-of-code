mod cache;
mod digits;
mod grid;
mod pos;
mod useful_parsers;

pub mod intcode;

pub use cache::*;
pub use digits::*;
pub use grid::*;
pub use pos::*;
pub use useful_parsers::*;

use std::{collections::HashMap, hash::Hash};

pub trait IteratorExt: Iterator + Sized {
    fn frequencies(self) -> HashMap<Self::Item, u32>
    where
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, item| {
            map.entry(item).and_modify(|c| *c += 1).or_insert(1);
            map
        })
    }
}

impl<I: Iterator + Sized> IteratorExt for I {}

#[macro_export]
macro_rules! iter {
    ($x:expr) => {
        std::array::IntoIter::new($x)
    };
}

pub use iter;
