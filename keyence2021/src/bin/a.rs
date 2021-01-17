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
        As: [usize; N],
        Bs: [usize; N],
    }

    let mut max_a_so_far = As[0];
    let mut c = Vec::with_capacity(N);
    c.push(As[0] * Bs[0]);

    for i in 1..N {
        max_a_so_far = max(max_a_so_far, As[i]);
        let prev_c = c[i - 1];
        c.push(max(prev_c, max_a_so_far * Bs[i]));
    }

    let ans = c
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", ans);
}
