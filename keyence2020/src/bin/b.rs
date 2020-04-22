#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xl: [(i64, i64); n],
    }
    xl.sort_by_key(|(x, l)| x - l);
    let mut remove_cnt = 0;
    let mut cur_end = xl[0].0 + xl[0].1;
    for i in 0..(n - 1) {
        let (nx, nl) = xl[i + 1];
        if cur_end > nx - nl {
            // overlap range exists
            remove_cnt += 1;
            cur_end = min(cur_end, nx + nl);
        } else {
            cur_end = nx + nl;
        }
    }
    println!("{}", n - remove_cnt);
}
