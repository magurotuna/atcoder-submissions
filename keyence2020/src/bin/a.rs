#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    if h >= w {
        println!("{}", (n + h - 1) / h);
    } else {
        println!("{}", (n + w - 1) / w);
    }
}
