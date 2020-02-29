use libprocon::*;

fn main() {
    input! {
        N: chars,
    }
    let N: Vec<i64> = N
        .into_iter()
        .map(|n| (n as usize - '0' as usize) as i64)
        .collect();

    use std::cmp::min;
    let mut eq = 0;
    let mut plus1 = 1;
    for i in 0..N.len() {
        let n = N[i];
        let e = eq;
        let p = plus1;
        eq = min(e + n, p + 10 - n);
        plus1 = min(e + n + 1, p + 10 - n - 1);
    }
    dbg!(eq, plus1);
    println!("{}", eq);
}
