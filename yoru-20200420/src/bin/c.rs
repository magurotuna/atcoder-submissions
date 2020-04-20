#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }
    let mut cnts = vec![0; n];
    for aa in a {
        cnts[aa] += 1;
    }
    cnts.sort();
    println!("{}", cnts.into_iter().take(n - k).sum::<usize>());
}
