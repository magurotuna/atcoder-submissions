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
        M: usize,
        ab: [(usize1, usize1); M],
    }
    let mut edges = Vec::new();
    for i in 0..N {
        edges.push(vec![]);
    }

    for i in 0..M {
        let a = ab[i].0;
        let b = ab[i].1;
        edges[a].push(b);
        edges[b].push(a);
    }

    use std::collections::HashSet;
    let mut visited = HashSet::new();
    visited.insert(0);

    println!("{}", dfs(&edges, &mut visited, 0));
}

fn dfs(edges: &Vec<Vec<usize>>, visited: &mut std::collections::HashSet<usize>, v: usize) -> usize {
    if visited.len() == edges.len() {
        return 1;
    }

    let mut cnt = 0;
    for &to in &edges[v] {
        if !visited.contains(&to) {
            let mut cl = visited.clone();
            cl.insert(to);
            cnt += dfs(edges, &mut cl, to);
        }
    }

    cnt
}
