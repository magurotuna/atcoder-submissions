#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
    }
    let mut cnt = 1;
    'o: for x in 1..2000000 {
        let mut xx = x;
        for _ in 0..d {
            if xx % 100 != 0 {
                continue 'o;
            }
            xx /= 100;
        }
        if xx % 100 == 0 {
            continue;
        }

        if cnt == n {
            println!("{}", x);
            return;
        }
        cnt += 1;
    }
}
