use libprocon::*;

fn solve(i: usize, j: usize, k: usize, N: usize, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if dp[i][j][k] >= 0.0 {
        return dp[i][j][k];
    }

    let choose_one = match i {
        0 => 0.0,
        _ => (i as f64 / N as f64) * solve(i - 1, j, k, N, dp),
    };
    let choose_two = match j {
        0 => 0.0,
        _ => (j as f64 / N as f64) * solve(i + 1, j - 1, k, N, dp),
    };
    let choose_three = match k {
        0 => 0.0,
        _ => (k as f64 / N as f64) * solve(i, j + 1, k - 1, N, dp),
    };

    let ret = (N as f64 / (i + j + k) as f64) * (choose_one + choose_two + choose_three + 1.0);
    dp[i][j][k] = ret;
    ret
}

fn main() {
    let N = read!(usize);
    let A: Vec<usize> = read![[usize]];

    let n_one = A.iter().filter(|&&x| x == 1).count();
    let n_two = A.iter().filter(|&&x| x == 2).count();
    let n_three = A.len() - n_one - n_two;

    // dp[i][j][k] :=
    // 1個の皿がi枚、2個の皿がj枚、3個の皿がk枚あるときの寿司がなくなるまでの操作回数の期待値
    let mut dp = vec![vec![vec![-1.0; N + 1]; N + 1]; N + 1];
    dp[0][0][0] = 0.0;

    println!("{}", solve(n_one, n_two, n_three, N, &mut dp));
}
