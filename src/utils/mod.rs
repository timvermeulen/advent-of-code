pub mod cache;
pub mod digits;
pub mod grid;
pub mod mask;
pub mod pos;
pub mod search;
pub mod useful_parsers;

pub mod fast_intcode;
pub mod intcode;

pub use cache::*;
pub use digits::*;
pub use grid::*;
pub use pos::*;
pub use search::*;
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

pub fn bits(mut n: usize) -> impl Iterator<Item = bool> {
    std::iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let bit = n % 2 == 1;
            n /= 2;
            Some(bit)
        }
    })
}

pub fn ascii_split(s: &str, byte: u8) -> impl Iterator<Item = &str> {
    s.as_bytes()
        .split(move |&b| b == byte)
        .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
}
