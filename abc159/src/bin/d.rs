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
    input! {
        N: usize,
        A: [i64; N],
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..A.len() {
        *map.entry(A[i]).or_insert(0i64) += 1;
    }

    let total_comb = map.iter().map(|(_, &v)| comb2(v)).sum::<i64>();
    let mut ans = Vec::with_capacity(N);
    for i in 0..N {
        let a = A[i];
        let &count = map.get(&a).unwrap();
        ans.push(total_comb - comb2(count) + comb2(count - 1));
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn comb2(n: i64) -> i64 {
    (n * (n - 1)) / 2
}
