use libprocon::*;
use std::collections::HashMap;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let MOD = 1_000_000_007;
    let mut fac = HashMap::new();

    // A1, A2, ..., AN の最小公倍数を求める（素因数分解の形で）
    for &a in A.iter() {
        let fac_a = a.factorize();
        for (k, v) in fac_a.into_iter() {
            match fac.get(&k) {
                Some(&n) if n < v => {
                    fac.insert(k, v);
                }
                Some(_) => (),
                None => {
                    fac.insert(k, v);
                }
            };
        }
    }

    let mut LCM = 1;
    for (k, v) in fac.into_iter() {
        LCM = (LCM * k.pow(v as u32)) % MOD;
    }

    let mut ans = 0;
    for &a in A.iter() {
        let tmp = (LCM * dbg!(mod_inv(a as u64, MOD as u64)) as usize) % MOD;
        ans += dbg!(tmp);
    }

    println!("{}", ans % MOD);
}
