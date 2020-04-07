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

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let (N, M) = {
        let mut it = buf.trim().split_whitespace();
        let N = it.next().unwrap().parse::<usize>().unwrap();
        let M = it.next().unwrap().parse::<usize>().unwrap();
        (N, M)
    };
    buf.clear();

    let mut prices = Vec::with_capacity(M);
    let mut cs = Vec::with_capacity(M);
    for i in 0..M {
        handle.read_line(&mut buf).unwrap();
        let (a, b) = {
            let mut it = buf.trim().split_whitespace();
            let a = it.next().unwrap().parse::<usize>().unwrap();
            let b = it.next().unwrap().parse::<usize>().unwrap();
            (a, b)
        };
        buf.clear();
        prices.push(a);

        handle.read_line(&mut buf).unwrap();
        cs.push(buf.trim().split_whitespace().map(|x| {
            let t = x.parse::<usize>().unwrap();
            1 << (t - 1)
        }).fold(0, |acc, cur| acc | cur ));
        buf.clear();
    }
    
    let inf = 1 << 60;
    let mut dp = vec![vec![inf; 1 << N]; M + 1];
    dp[0][0] = 0;

    use std::cmp::min;
    for i in 0..M {
        for j in 0..(1 << N) {
            // use the i-th key
            dp[i+1][j | cs[i]] = min(dp[i+1][j | cs[i]], dp[i][j] + prices[i]);

            // not use the i-th key
            dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
        }
    }

    let ans = dp[M][(1 << N) - 1];
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
