use itertools::Itertools;
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }
    let n = a1 + a2 + a3;
    let mut ans = 0;
    'outer: for p in (1..=n).permutations(n) {
        let a = &p[..a1];
        let b = &p[a1..(a1 + a2)];
        let c = &p[(a1 + a2)..];
        for i in 0..(a.len() - 1) {
            if a[i] >= a[i + 1] {
                continue 'outer;
            }
        }
        for i in 0..(b.len() - 1) {
            if b[i] >= b[i + 1] {
                continue 'outer;
            }
        }
        for i in 0..(c.len() - 1) {
            if c[i] >= c[i + 1] {
                continue 'outer;
            }
        }

        for i in 0..min(a1, a2) {
            if a[i] >= b[i] {
                continue 'outer;
            }
        }
        for i in 0..min(a2, a3) {
            if b[i] >= c[i] {
                continue 'outer;
            }
        }

        ans += 1;
    }
    println!("{}", ans);
}
