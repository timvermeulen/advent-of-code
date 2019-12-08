use crate::{input::*, utils::*};
use parser::prelude::*;
use std::{
    cell::{Cell, RefCell},
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    iter::{self, successors, FromIterator},
    mem,
    ops::Range,
};

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2019_optimized;
