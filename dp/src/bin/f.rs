use libprocon::*;

fn main() {
    let s: String = read!(String);
    let t: String = read!(String);

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    // dp[i][j] := s1...siとt1...tjに対する共通部分列
    let mut dp = vec![vec![0usize; t.len() + 1]; s.len() + 1];

    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }

    // LCSの復元
    let mut lcs_rev = String::new();
    let mut x = s.len();
    let mut y = t.len();
    loop {
        if dp[x][y] == 0 {
            break;
        }

        let cur = dp[x][y];
        if cur == dp[x - 1][y] {
            x = x - 1;
        } else if cur == dp[x][y - 1] {
            y = y - 1;
        } else {
            lcs_rev.push(s[x - 1]);
            x = x - 1;
            y = y - 1;
        }
    }
    println!("{}", lcs_rev.chars().rev().collect::<String>());
}
