//use libprocon::*;
use libprocon::{read, rl};

fn main() {
    let MOD = 1_000_000_007;
    let (H, W) = read!(usize, usize);
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(H);
    for _ in 0..H {
        let row = read!(String).chars().collect::<Vec<_>>();
        grid.push(row);
    }

    // dp[i][j] := (i, j)への経路の数
    let mut dp = vec![vec![0; W]; H];
    dp[0][0] = 1;
    for i in 0..H {
        for j in 0..W {
            if i == 0 && j == 0 {
                continue;
            }
            if grid[i][j] == '#' {
                continue;
            }

            let from_upper = match j {
                0 => 0,
                _ => dp[i][j - 1] % MOD,
            };
            let from_left = match i {
                0 => 0,
                _ => dp[i - 1][j] % MOD,
            };

            dp[i][j] = (from_upper + from_left) % MOD;
        }
    }

    println!("{}", dp[H - 1][W - 1]);
}
