use libprocon::*;

fn main() {
    input! {
        N: usize,
        W: [i64; N],
    }

    let mut ans: i64 = 1 << 60;
    for i in 1..N {
        let s1: i64 = W.iter().take(i).sum();
        let s2: i64 = W.iter().skip(i).sum();
        ans = std::cmp::min(ans, (s1 - s2).abs());
    }
    println!("{}", ans);
}
