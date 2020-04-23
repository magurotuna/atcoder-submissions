#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut b: [i64; n],
    }
    let mut v = VecDeque::new();
    for _ in 0..n {
        let mut max_idx = n + 1;
        let mut cnt = 1;
        for j in 0..n {
            if b[j] == -1 {
                continue;
            }
            if cnt == b[j] {
                max_idx = j;
            }
            cnt += 1;
        }
        if max_idx == n + 1 {
            println!("-1");
            return;
        }
        v.push_front(b[max_idx].to_string());
        b[max_idx] = -1;
    }
    println!("{}", v.into_iter().collect::<Vec<_>>().join("\n"));
}
