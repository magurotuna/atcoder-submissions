use libprocon::*;
use std::cmp::min;

#[derive(Debug, Clone, Copy)]
struct Magic {
    damage: usize,
    point: usize,
}

fn main() {
    let INF: usize = 1 << 60;
    let (hp, N) = read!(usize, usize);
    let mut ab: Vec<Magic> = vec![];
    for i in 0..N {
        let (damage, point) = read!(usize, usize);
        ab.push(Magic { damage, point });
    }
    // 価値の総和がH以上となるように選んだときの、重さの最小値を求めよ。
    // dp[i+1][h] :=
    // i番目の攻撃方法の中から価値の総和がh以上となるように選んだときの、魔力の総和の最小値
    let mut dp = vec![vec![INF; hp + 1]; N + 1];

    dp[0][0] = 0;

    for i in 0..N {
        for j in 0..=hp {
            // i+1番目を使用しない場合の魔力総和
            let not_use_i = dp[i][j];
            // i+1番目を使用する場合の魔力総和
            let use_i = dp[i + 1][j.saturating_sub(ab[i].damage)] + ab[i].point;

            dp[i + 1][j] = min(not_use_i, use_i);
        }
    }
    println!("{}", dp[N][hp]);
}
