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
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    iter::{self, successors, FromIterator},
    mem,
    ops::{Index, IndexMut, Range},
};

mod year2015;
mod year2016;
mod year2017;
pub mod search_algs {
    pub use pathfinding::prelude::{astar, bfs, dfs, dijkstra};
}
mod year2018;
mod year2019;
mod year2019_optimized;
