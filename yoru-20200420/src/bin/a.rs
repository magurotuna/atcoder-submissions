#![allow(unused_imports)]

use permutohedron::heap_recursive;
use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        a: [usize; 5],
    }
    let mut data = Vec::new();
    for i in 0..5 {
        data.push(i);
    }
    let mut p = Vec::new();
    heap_recursive(&mut data, |perm| p.push(perm.to_vec()));

    let mut ans = 1 << 60;
    use std::cmp::min;

    for pp in &p {
        let mut time = 0;
        for i in 0..pp.len() {
            let ppp = pp[i];
            time += a[ppp];
            if i < pp.len() - 1 && a[ppp] % 10 > 0 {
                time += 10 - (a[ppp] % 10);
            }
        }
        ans = min(ans, time);
    }
    println!("{}", ans);
}
