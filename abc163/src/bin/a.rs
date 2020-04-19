#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        r: f64,
    }
    println!("{}", 2.0 * r * std::f64::consts::PI);
}
