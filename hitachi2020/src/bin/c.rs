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
        ab: [(usize1, usize1); N-1],
    }
    let mut links = vec![vec![]; N];
    for i in 0..(N - 1) {
        let a = ab[i].0;
        let b = ab[i].1;
        links[a].push(b);
        links[b].push(a);
    }

    use std::collections::HashSet;
    let mut dist3 = Vec::with_capacity(N);
    for i in 0..N {
        dist3.push((i, HashSet::new()));
    }
    for i in 0..N {
        let mut visited = HashSet::new();
        dfs(&mut dist3, 0, i, i, &links, &mut visited);
    }

    dist3.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    dbg!(&dist3);
    let mut mod0 = Vec::with_capacity(N);
    let mut mod1 = Vec::with_capacity(N);
    let mut mod2 = Vec::with_capacity(N);
    for i in 1..=N {
        match i % 3 {
            0 => mod0.push(i),
            1 => mod1.push(i),
            2 => mod2.push(i),
            _ => unreachable!(),
        }
    }

    let mut ans = vec![1 << 60; N];
    for i in 0..N {
        if dist3[i].1.is_empty() {
            let p = if !mod0.is_empty() {
                mod0.pop().unwrap()
            } else if !mod1.is_empty() {
                mod1.pop().unwrap()
            } else {
                mod2.pop().unwrap()
            };
            ans[dist3[i].0] = p;
        } else {
        }
    }
    dbg!(&mod0);
}

fn dfs(
    dist3: &mut Vec<(usize, std::collections::HashSet<usize>)>,
    cur_dist: usize,
    cur_node: usize,
    root_node: usize,
    links: &Vec<Vec<usize>>,
    visited: &mut std::collections::HashSet<usize>,
) {
    use std::cmp::{max, min};
    visited.insert(cur_node);
    if cur_dist == 3 {
        // dbg!((cur_dist, cur_node, root_node));
        dist3[root_node].1.insert(cur_node);
        dist3[cur_node].1.insert(root_node);
        return;
    }
    let cur_dist = cur_dist + 1;
    for &child in links[cur_node].iter() {
        if visited.contains(&child) {
            continue;
        }
        dfs(dist3, cur_dist, child, root_node, links, visited);
    }
}
