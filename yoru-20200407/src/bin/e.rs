use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let (N, M) = {
        let t = buf.trim()
    .split_whitespace()
    .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (t[0], t[1])
    };
    buf.clear();

    let mut prices = Vec::with_capacity(M);
    let mut cs = Vec::with_capacity(M);
    for _ in 0..M {
        handle.read_line(&mut buf).unwrap();
        let (a, b) = {
            let t = buf.trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            buf.clear();
            (t[0], t[1])
        };
        prices.push(a);

        handle.read_line(&mut buf).unwrap();
        cs.push(buf.trim().split_whitespace().map(|x| {
            let num = x.parse::<usize>().unwrap();
            1 << (num - 1)
        }).fold(0, |acc, cur| acc | cur));
        buf.clear();
    }

    let inf = 1 << 60;
    let mut dp = vec![vec![inf; 1 << N]; M + 1];
    dp[0][0] = 0;

    use std::cmp::min;
    for i in 0..M {
        for j in 0..(1 << N) {
            if dp[i][j] == inf {
                continue;
            }
            // 鍵 i を使う場合
            dp[i + 1][cs[i] | j] = min(dp[i + 1][cs[i] | j], dp[i][j] + prices[i]);
            // 使わない場合
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }

    let mut ans = inf;
    for i in 0..(M+1) {
        ans = min(ans, dp[i][(1 << N) - 1]);
    }

    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
