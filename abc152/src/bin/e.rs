use libprocon::*;

// TODO: Wrong Answer (11/18)
fn main() {
    input! {
        N: usize,
        A: [u128; N],
    }

    // A1, A2, ..., AN の最小公倍数を求める
    let mut LCM = A[0];
    for &a in A.iter() {
        LCM = lcm(a, LCM);
    }

    let MOD = 1_000_000_007;
    let mut ans = 0;
    for &a in A.iter() {
        let b = LCM / a;
        ans = (ans + b) % MOD;
    }
    println!("{}", ans);
}

pub fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}
