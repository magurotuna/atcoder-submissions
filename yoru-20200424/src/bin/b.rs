#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }
    h.sort();
    use std::cmp::min;
    let mut ans = usize::MAX;
    for i in 0..(n - k + 1) {
        ans = min(ans, h[i + k - 1] - h[i]);
    }
    println!("{}", ans);
}
