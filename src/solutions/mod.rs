pub use crate::{input::*, utils::*};
pub use num::integer::{gcd, lcm};
pub use parser::prelude::*;
pub use rayon::prelude::*;
pub use std::{
    cell::{Cell, RefCell},
    cmp::{self, Ordering},
    collections::{
        hash_map::Entry::{self, *},
        BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    },
    fmt::{self, Debug, Display, Formatter, Write},
    hash::Hash,
    iter::{self, successors, FromIterator},
    mem,
    ops::{Index, IndexMut, Range},
};

pub mod search_algs {
    pub use pathfinding::prelude::{astar, bfs, dfs, dijkstra};
}

// pub mod year2015;
// pub mod year2016;
// pub mod year2017;
// pub mod year2018;
pub mod year2019;
pub mod year2019_optimized;

mod bench;
