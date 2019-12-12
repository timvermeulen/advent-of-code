pub use crate::{input::*, utils::*};
pub use num::integer::{gcd, lcm};
pub use parser::prelude::*;
pub use rayon::prelude::*;
pub use std::{
    cell::{Cell, RefCell},
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt::{self, Debug, Formatter},
    hash::Hash,
    iter::{self, successors, FromIterator},
    mem,
    ops::{Index, IndexMut, Range},
};

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2019_optimized;
