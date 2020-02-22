use libprocon::*;
use std::cmp::min;

fn main() {
    input! {
        N: usize,
        X: [i64; N],
    }

    let mut ans = 1 << 60;
    for p in 1..=100 {
        let total = X.iter().map(|&x| (x - p).pow(2u32)).sum::<i64>();
        ans = min(total, ans);
    }
    println!("{}", ans);
}
