#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }
    use std::cmp::Reverse;
    a.sort();
    b.sort();
    c.sort();
    let mut s = Vec::new();
    s.push(0);
    for i in 0..n {
        let b = b[i];
        let upper = {
            let mut ok = n as i64;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if a[mid as usize] >= b {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize
        };
        let last = *s.last().unwrap();
        s.push(last + upper);
    }

    let mut ans = 0;

    for i in 0..n {
        let c = c[i];
        let upper = {
            let mut ok = n as i64;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if b[mid as usize] >= c {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize
        };
        ans += s[upper];
    }
    println!("{}", ans);
}
