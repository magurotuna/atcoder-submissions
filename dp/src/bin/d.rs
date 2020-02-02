use libprocon::*;

fn main() {
    let (N, W) = read!(usize, usize);
    let mut wv: Vec<(usize, usize)> = Vec::with_capacity(N);
    for _ in 0..N {
        wv.push(read!(usize, usize));
    }

    // dp[i+1][w] := 番号iまでの品物の中から重さw以下になるように選ぶときの最大の価値
    let mut dp = vec![vec![0usize; W + 1]; N + 1];

    for i in 0..N {
        for j in 0..=W {
            // iを使わない場合の価値
            let not_used = dp[i][j];
            // iを使う場合の価値
            let used = if wv[i].0 <= j {
                dp[i][j - wv[i].0] + wv[i].1
            } else {
                0
            };
            dp[i + 1][j] = std::cmp::max(not_used, used);
        }
    }

    println!("{}", dp[N][W]);
}
