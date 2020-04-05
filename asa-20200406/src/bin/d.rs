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
        abc: [(usize1, usize1, i64); N-1],
        Q: usize,
        K: usize1,
        xy: [(usize1, usize1); Q],
    }

    let mut neighbors = Vec::with_capacity(N);
    for i in 0..N {
        neighbors.push(vec![]);
    }
    for i in 0..(N-1) {
        let (from, to, dist) = abc[i];
        neighbors[from].push((to, dist));
        neighbors[to].push((from, dist));
    }

    use std::collections::{VecDeque, HashSet};
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((K, 0));

    let mut d_from_k = vec![0; N];

    while let Some((v, dist)) = queue.pop_front() {
        visited.insert(v);        
        d_from_k[v] = dist;

        for &(child, d) in neighbors[v].iter() {
            if !visited.contains(&child) {
                queue.push_back((child, dist + d));
            }
        }
    }
    
    let mut ans = Vec::with_capacity(Q);
    for i in 0..Q {
        let (x, y) = xy[i];
        ans.push((d_from_k[x] + d_from_k[y]).to_string());
    }

    println!("{}", ans.join("\n"));
}
