#![allow(
    unused_imports,
    unused_attributes,
    unused_macros,
    dead_code,
    non_snake_case
)]
use proconio::input;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;

fn main() {
    input! {
        N: usize,
        X: usize,
        T: usize,
    }

    let ans = ((N + X - 1) / X) * T;
    println!("{}", ans);
}
