use libprocon::*;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut ans = 0;
    use std::cmp::{max, min};
    for i in 1..(n - 1) {
        if min(p[i - 1], p[i + 1]) < p[i] && p[i] < max(p[i - 1], p[i + 1]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
