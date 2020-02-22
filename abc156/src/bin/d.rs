use libprocon::*;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }
    let MOD = 1_000_000_007;

    // 2^n - 1 通りの花束を作れる
    // そこから n_C_a と n_C_b を除けばOK
    let all = mod_pow(2, n, MOD);
    let na = calc(n, a, MOD);
    let nb = calc(n, b, MOD);
    dbg!((all, na, nb));

    let mut ans = (all as i64) - 1 - (na as i64) - (nb as i64);
    ans %= MOD as i64;
    if ans < 0 {
        ans += MOD as i64;
    }
    println!("{}", ans);
}

fn calc(n: u64, k: u64, m: u64) -> u64 {
    let mut val = 1;
    for i in 0..k {
        val %= m;
        val *= (n - i) % m;
        val %= m;
        val *= mod_inv(i + 1, m);
    }
    val % m
}
