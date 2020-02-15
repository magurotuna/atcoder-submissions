use libprocon::*;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let MOD = 1_000_000_007_usize;
    let comb = Comb::new(10000, MOD);
    for i in 1..=k {
        println!(
            "{}",
            comb.calc(n - k + 1, i) * comb.calc(k - 1, i - 1) % MOD
        );
    }
}
