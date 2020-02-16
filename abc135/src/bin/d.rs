use libprocon::*;

fn main() {
    input! {
        s: chars,
    }
    let MOD = 1_000_000_007usize;

    // dp[i][j] := 上位i文字として考えられる数字のうち、13で割ったあまりがjであるものの数
    let mut dp = vec![vec![0; 13]; s.len() + 1];
    dp[0][0] = 1;

    for i in 1..=s.len() {
        if s[i - 1] == '?' {
            for j in 0..13 {
                for q in 0..10 {
                    let r = (10 * j + q) % 13;
                    dp[i][r] += dp[i - 1][j] % MOD;
                }
            }
        } else {
            for j in 0..13 {
                let r = (10 * j + (s[i - 1] as usize - '0' as usize)) % 13;
                dp[i][r] += dp[i - 1][j] % MOD;
            }
        }
    }
    println!("{}", dp[s.len()][5] % MOD);
}
