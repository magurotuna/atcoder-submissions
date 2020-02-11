use libprocon::*;
use std::cmp::{max, min};

// 解説AC
// 自力ではrest（追放できる回数）を min(N, K) - l としてしまっていた 惜しい
fn main() {
    input! {
        N: usize,
        K: usize,
        V: [i64; N],
    }

    let mut ans = 0_i64;
    for l in 1..=min(N, K) {
        for start in (N - l)..=N {
            let mut rest = K - l; // rest回だけ価値の低い宝石を追放できる
            let mut arr = V.iter().cycle().skip(start).take(l).collect::<Vec<_>>();
            arr.sort();
            let mut range_sum = 0;
            for x in arr {
                if rest > 0 && *x < 0 {
                    rest -= 1;
                    continue;
                }
                range_sum += x;
            }
            dbg!((start, l, range_sum));
            ans = max(ans, range_sum);
        }
    }
    println!("{}", ans);
}
