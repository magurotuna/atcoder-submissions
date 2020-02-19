use libprocon::*;
use std::cmp::max;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [[usize; M]; N],
    }
    let mut ans = 0;
    for i in 0..M {
        for j in i..M {
            let point = (0..N).map(|n| max(A[n][i], A[n][j])).sum::<usize>();
            ans = max(ans, point);
        }
    }
    println!("{}", ans);
}
