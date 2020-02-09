use libprocon::*;
use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
        M: usize,
        a: [usize; M],
    }
    let MOD = 1_000_000_007;

    let hs: HashSet<usize> = a.into_iter().collect();

    // dp[i] := i段目にたどりつく移動方法の数
    let mut dp = vec![0; N + 1];
    dp[0] = 1;
    for i in 1..=N {
        if hs.contains(&i) {
            dp[i] = 0;
        } else {
            let minus_1 = *dp.get(i - 1).unwrap_or(&0);
            let minus_2 = if i == 1 {
                0
            } else {
                *dp.get(i - 2).unwrap_or(&0)
            };
            dp[i] = (minus_1 + minus_2) % MOD;
        }
    }
    println!("{}", dp[N]);
}
