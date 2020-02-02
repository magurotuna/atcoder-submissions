use libprocon::*;

fn main() {
    let N = read!(usize);
    let p: Vec<f64> = read![[f64]];

    // dp[i][c] := i番目までのコインをなげたときに表がc枚出ている確率
    let mut dp = vec![vec![None; N + 1]; N + 1];
    dp[0][0] = Some(1.0);

    for i in 1..=N {
        for c in 0..=i {
            let front = match c {
                0 => 0.0,
                x => dp[i - 1][x - 1].unwrap() * p[i - 1],
            };
            let back = match dp[i - 1][c] {
                Some(x) => x * (1.0 - p[i - 1]),
                None => 0.0,
            };
            dp[i][c] = Some(front + back);
        }
    }

    let ans = dp[N]
        .iter()
        .filter_map(|&x| x)
        .skip((N + 1) / 2)
        .sum::<f64>();
    println!("{}", ans);
}
