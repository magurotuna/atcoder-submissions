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

fn read_vec2(n: u32) -> Vec<Vec<char>> {
    let mut v2 = Vec::with_capacity(n as usize);
    use std::io::stdin;
    use std::io::BufRead;
    let stdin = stdin();
    let mut stdin = stdin.lock();
    for _ in 0..n {
        let mut s = String::new();
        stdin.read_line(&mut s).ok();
        let v = s.trim().chars().filter(|&c| c != ' ').collect();
        v2.push(v);
    }
    v2
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let S: String = read();
    let S: Vec<char> = S.chars().collect();
    let Q: usize = read();
    let query = read_vec2(Q as u32);
    use std::collections::VecDeque;
    let mut S: VecDeque<_> = S.into();
    S.reserve(Q);
    let mut reverse = false;

    for i in 0..Q {
        let q = &query[i];
        if q[0] == '1' {
            reverse = !reverse;
        } else {
            let f = q[1];
            let c = q[2];
            if (!reverse && f == '1') || (reverse && f == '2') {
                S.push_front(c);
            } else if (!reverse && f == '2') || (reverse && f == '1') {
                S.push_back(c);
            }
        }
    }
    let ans: String = if reverse {
        let mut S: Vec<_> = S.into();
        S.reverse();
        S.into_iter().collect()
    } else {
        S.into_iter().collect()
    };
    println!("{}", ans);
}
