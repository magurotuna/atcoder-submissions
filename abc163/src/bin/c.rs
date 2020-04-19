#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    }
    let mut v = vec![0; n + 3];
    for i in 0..a.len() {
        let boss = a[i];
        v[boss] += 1;
    }
    println!(
        "{}",
        v.into_iter()
            .skip(1)
            .take(n)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
