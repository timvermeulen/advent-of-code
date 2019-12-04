use crate::{input::*, utils::*};
use parser::prelude::*;
use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    iter::{self, FromIterator},
    mem,
    ops::Range,
};

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2019_optimized;
