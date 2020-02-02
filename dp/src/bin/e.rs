use libprocon::*;

fn main() {
    let (N, W) = read!(usize, usize);
    let mut wv: Vec<(usize, usize)> = Vec::with_capacity(N);
    for _ in 0..N {
        wv.push(read!(usize, usize));
    }

    let inf: usize = 1 << 60;
    let max_v = 1000;
    // dp[i+1][v] := 番号iまでの品物の中から価値の総和がvとなるように選んだときの重さの総和の最小値
    let mut dp = vec![vec![inf; max_v * N + 1]; N + 1];
    dp[0][0] = 0;

    for i in 0..N {
        for v in 0..=(max_v * N) {
            let not_used = dp[i][v];
            let used = if v >= wv[i].1 {
                dp[i][v - wv[i].1] + wv[i].0
            } else {
                inf
            };
            dp[i + 1][v] = std::cmp::min(not_used, used);
        }
    }

    let ans = dp[N]
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x <= W { Some(i) } else { None })
        .max()
        .unwrap();
    println!("{}", ans);
}
