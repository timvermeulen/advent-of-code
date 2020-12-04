use num::traits::Zero;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

pub trait Dijkstra {
    type Node: Eq + Hash + Clone;
    type Cost: Copy + Ord + Zero;

    fn start(&self) -> Self::Node;
    fn is_goal(&self, node: &Self::Node) -> bool;
    fn neighbors(&self, node: &Self::Node, add: impl FnMut(Self::Node, Self::Cost));

    fn run(&self) -> Option<Self::Cost> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        heap.push(Wrapper { node: self.start(), cost: Self::Cost::zero() });
        // let mut count = 0;
        // let mut max_len = 0;

        while let Some(Wrapper { node, cost }) = heap.pop() {
            if !seen.insert(node.clone()) {
                continue;
            }
            // count += 1;
            if self.is_goal(&node) {
                // dbg!(count);
                // dbg!(max_len);
                return Some(cost);
            }
            self.neighbors(&node, |node, c| heap.push(Wrapper { node, cost: cost + c }));
            // max_len = std::cmp::max(max_len, heap.len());
        }
        None
        // let (_, cost) = dijkstra(
        //     &self.start(),
        //     |node| {
        //         let mut vec = Vec::new();
        //         self.neighbors(node, |neighbor, cost| vec.push((neighbor, cost)));
        //         vec
        //     },
        //     |node| self.is_goal(node),
        // )?;
        // Some(cost)
    }
}

struct Wrapper<N, C> {
    node: N,
    cost: C,
}

impl<N, C> PartialEq for Wrapper<N, C>
where
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N, C> Eq for Wrapper<N, C> where C: Eq {}

impl<N, C> Ord for Wrapper<N, C>
where
    C: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<N, C> PartialOrd for Wrapper<N, C>
where
    C: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
