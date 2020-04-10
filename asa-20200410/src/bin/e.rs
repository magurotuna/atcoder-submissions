#![allow(unused_macros)]

// cf. [Rustで競技プログラミングの入力をスッキリ記述するマクロ - Qiita](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// TODO
fn main() {
    input! {
        N: chars,
        K: usize,
    }
    let mut dp = vec![vec![vec![0; 2]; K + 2]; N.len() + 1];
    dp[0][0][0] = 1;

    for i in 0..N.len() {
        for j in 0..(K + 1) {
            let n = (N[i] as u8 - '0' as u8) as usize;
            // 0を使うとき
            if n == 0 {
                dp[i + 1][j][0] += dp[i][j][0];
            } else {
                dp[i + 1][j][1] += dp[i][j][0] + dp[i][j][1];
            }

            for d in 1..10 {
                println!("d: {} n: {}", d, n);
                if d == n {
                    dp[i + 1][j + 1][0] += dp[i][j][0];
                } else if d < n {
                    dp[i + 1][j + 1][1] += dp[i][j][0] + dp[i][j][1];
                } else {
                    dp[i + 1][j + 1][1] += dp[i][j][1];
                }
            }
        }
    }
    println!("{:?}", &dp);
}
