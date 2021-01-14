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
        N: String,
    }

    let mut s = 0;
    for c in N.chars() {
        let c = c.to_string();
        let n = c.parse::<i32>().unwrap();
        s += n;
    }
    if s % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
