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
    }
    let mut ans = 0;
    let mut prev_max = As[0];
    for a in As.into_iter().skip(1) {
        if prev_max > a {
            ans += prev_max - a;
        } else {
            prev_max = a;
        }
    }
    println!("{}", ans);
}
