use libprocon::*;
use std::cmp::max;

fn solve(l: usize, r: usize, a: &[i64], dp: &mut Vec<Vec<Option<i64>>>) -> i64 {
    if let Some(x) = dp[l][r] {
        x
    } else {
        let left = a[l] - solve(l + 1, r, a, dp);
        let right = a[r - 1] - solve(l, r - 1, a, dp);
        let ret = max(left, right);
        dp[l][r] = Some(ret);
        ret
    }
}

fn main() {
    input! {
        N: usize,
        a: [i64; N],
    }

    // dp[l][r] := aの[l, r)が残っている状態でゲームが開始したとして、最適に行動したときの先手-後手の値
    let mut dp = vec![vec![None; N + 1]; N + 1];
    // 求めたいもの dp[0][N]
    // r = l + 1 のときはDequeに1要素しか残っていないのでその残った要素の数字が先手-後手の値となる
    for i in 0..N {
        dp[i][i] = Some(0_i64);
        dp[i][i + 1] = Some(a[i]);
    }
    println!("{}", solve(0, N, &a, &mut dp));
}
