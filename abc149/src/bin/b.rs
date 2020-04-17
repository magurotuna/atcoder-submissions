use libprocon::*;

fn main() {
    input! {
        A: i64,
        B: i64,
        K: i64,
    }
    use std::cmp::{max, min};
    println!("{} {}", max(0, A - K), max(0, B - max(0, K - A)));
}
