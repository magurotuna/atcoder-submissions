#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: usize,
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if i < k {
            ans.push(s);
        } else {
            ans.push(if s == 1_000_000_000 { 1 } else { s + 1 });
        }
    }
    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
