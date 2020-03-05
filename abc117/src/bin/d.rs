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

fn main() {
    use std::cmp::max;
    input! {
        N: usize,
        K: i64,
        A: [i64; N],
    }

    let mut binary_one_counts = vec![0; 60]; // the appearance counts of one in the binary representation
    for &a in A.iter() {
        for i in 0..60 {
            if 1 << i & a != 0 {
                binary_one_counts[i] += 1;
            }
        }
    }

    // dp[d][tight] := 上位d桁目まで見て、今の桁まで K と同じ(tight = 1)かどうか、という場合の最大値
    let mut dp = vec![vec![-1i64; 2]; 67];
    dp[0][0] = 0;

    for i in 0..60 {
        let one_count = binary_one_counts[60 - i - 1] as i64;
        let mask = 1 << (60 - i - 1);
        // X の i 桁目を 0, 1 にしたときの利益
        let profit0 = mask * one_count;
        let profit1 = mask * (N as i64 - one_count);

        // smaller -> smaller
        if dp[i][1] != -1 {
            dp[i + 1][1] = max(dp[i + 1][1], dp[i][1] + max(profit0, profit1)); // すでにsmallerが確定してるから0, 1どちらも可能
        }

        // exact -> smaller
        if dp[i][0] != -1 {
            // K の i 桁目が立っているなら、X の i 桁目は0にする
            if K & mask != 0 {
                dp[i + 1][1] = max(dp[i + 1][1], dp[i][0] + profit0);
            }
        }

        // exact -> exact
        if dp[i][0] != -1 {
            // K の i 桁目が立っているなら X の i 桁目も立たせる。逆もまた然り
            dp[i + 1][0] = if K & mask != 0 {
                max(dp[i + 1][0], dp[i][0] + profit1)
            } else {
                max(dp[i + 1][0], dp[i][0] + profit0)
            };
        }
    }
    println!("{}", max(dp[60][0], dp[60][1]));
}
