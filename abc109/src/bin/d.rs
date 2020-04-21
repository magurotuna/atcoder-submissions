#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        mut a: [[i64; W]; H],
    }
    let mut cnt = 0;
    let mut manipulates = Vec::new();
    for h in 0..H {
        if h % 2 == 0 {
            for w in 0..W {
                if h == H - 1 && w == W - 1 {
                    continue;
                }
                let nh = if w == W - 1 { h + 1 } else { h };
                let nw = if w == W - 1 { w } else { w + 1 };
                if a[h][w] % 2 != 0 {
                    cnt += 1;
                    manipulates.push(format!("{} {} {} {}", h + 1, w + 1, nh + 1, nw + 1));
                    a[nh][nw] += 1;
                }
            }
        } else {
            for w in (0..W).rev() {
                if h == H - 1 && w == 0 {
                    continue;
                }
                let nh = if w == 0 { h + 1 } else { h };
                let nw = if w == 0 { w } else { w - 1 };
                if a[h][w] % 2 != 0 {
                    cnt += 1;
                    manipulates.push(format!("{} {} {} {}", h + 1, w + 1, nh + 1, nw + 1));
                    a[nh][nw] += 1;
                }
            }
        }
    }
    println!("{}", cnt);
    println!("{}", manipulates.join("\n"));
}
