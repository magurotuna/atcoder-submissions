use libprocon::*;
use num_bigint::BigInt;

// Use num_bigint crate. AC (1939ms)
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // A1, A2, ..., AN の最小公倍数を求める
    let mut LCM = BigInt::from(A[0]);
    for &a in A.iter() {
        LCM = lcm(BigInt::from(a), LCM);
    }

    let MOD = BigInt::from(1_000_000_007);
    let mut ans = BigInt::from(0);
    for &a in A.iter() {
        let b = LCM.clone() / a;
        ans = (ans + b) % MOD.clone();
    }
    println!("{}", ans);
}

pub fn gcd(a: BigInt, b: BigInt) -> BigInt {
    if b == BigInt::from(0) {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

pub fn lcm(a: BigInt, b: BigInt) -> BigInt {
    a.clone() / gcd(a, b.clone()) * b
}
