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
        mut K: usize,
        As: [usize; N],
    }

    let mut counts = vec![0usize; N];
    for a in As {
        counts[a] += 1;
    }

    let mut ans = 0;

    for i in 0..N {
        let cnt = counts[i];
        if K > cnt {
            ans += i * (K - cnt);
            K = cnt;
        }
    }

    println!("{}", ans);
}
