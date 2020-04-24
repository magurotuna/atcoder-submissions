#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    if n % 2 == 0 {
        println!("0.5");
    } else {
        let n = n as f64;
        println!("{}", (n + 1.0) / (2.0 * n));
    }
}
