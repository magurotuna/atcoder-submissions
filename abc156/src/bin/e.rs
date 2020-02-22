use libprocon::*;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let MOD = 1_000_000_007;

    let comb = Comb::new(300_000, MOD);

    let mut ans = 1;
    for i in 1..=(min(n - 1, k)) {
        ans += comb.calc(n, i) * comb.calc(n - 1, n - i - 1);
        ans %= MOD;
    }
    println!("{}", ans % MOD);
}
