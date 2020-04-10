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

use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        ab: [(usize1, usize1); N-1],
    }
    let mut edges = Vec::with_capacity(N);
    for _ in 0..N {
        edges.push(vec![]);
    }
    for &(a, b) in &ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    use std::collections::{VecDeque, HashSet};
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut d_from_zero = vec![0; N];
    queue.push_back((0, 0));

    while let Some((cur, dist)) = queue.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        d_from_zero[cur] = dist;
        visited.insert(cur);

        for &to in &edges[cur] {
            queue.push_back((to, dist + 1));
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((N - 1, 0));
    let mut s_cnt = 0;

    while let Some((cur, dist)) = queue.pop_front() {
        if visited.contains(&cur) {
            continue;
        }

        visited.insert(cur);
        if dist < d_from_zero[cur] {
            s_cnt += 1;
        }

        for &to in &edges[cur] {
            queue.push_back((to, dist + 1));
        }
    }

    let f_cnt = N - s_cnt;

    println!("{}", if f_cnt > s_cnt {"Fennec"} else {"Snuke"});
}

