#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let s = a.into_iter().sum::<usize>();
    if n >= s {
        println!("{}", n - s);
    } else {
        println!("-1");
    }
}
