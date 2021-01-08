#![allow(unused)]
#![feature(
    test,
    array_value_iter,
    try_blocks,
    box_syntax,
    core_intrinsics,
    partition_point,
    exact_size_is_empty,
    array_windows,
    array_chunks
)]

mod input;
mod solutions;
mod utils;

use solutions::*;

extern crate crypto;
extern crate iterslide;

use crypto::{digest::Digest, md5::Md5};
use iterslide::SlideIterator;

const SALT: &'static str = "ihaygndm";

fn hash(salt: &str, n: u32, loops: usize) -> String {
    let mut md5 = Md5::new();
    md5.input_str(salt);
    md5.input_str(&n.to_string());
    let mut result = md5.result_str();
    for _ in 0..loops {
        md5.reset();
        md5.input_str(&result);
        result = md5.result_str();
    }
    result
}

fn threes(s: &str) -> Option<char> {
    s.chars()
        .slide(3)
        .find(|ss| ss[0] == ss[1] && ss[1] == ss[2])
        .map(|ss| ss[0])
}

fn has_five(s: &str, c: char) -> bool {
    s.chars().slide(5).any(|ss| ss.iter().all(|cc| c == *cc))
}

fn p14(salt: &str, mut index: u32, loops: usize) -> (u32, u32) {
    for (three, five) in (0..100000)
        .map(|n| {
            let s = hash(salt, n, loops);
            (s, n)
        })
        .slide(1001)
        .filter_map(|v| {
            if let Some(c) = threes(&v[0].0) {
                v.iter()
                    .skip(1)
                    .find(|&(ref k, _)| has_five(&k, c))
                    .map(|&(_, x)| (v[0].1, x))
            } else {
                None
            }
        })
    {
        if index == 0 {
            return (three, five);
        } else {
            index -= 1;
        }
    }
    panic!()
}

fn main() {
    println!("P1 = {:?}", p14(SALT, 63, 0));
    println!("P2 = {:?}", p14(SALT, 63, 2016));
}
