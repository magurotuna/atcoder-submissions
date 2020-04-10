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
        Q: usize,
        ab: [(usize1, usize1); N-1],
        px: [(usize1, i64); Q],
    }

    let mut edges = Vec::with_capacity(N);
    let mut de = Vec::with_capacity(N);
    for _ in 0..N {
        edges.push(vec![]);
        de.push(vec![]);
    }
    for i in 0..(N - 1) {
        let (a, b) = ab[i];
        edges[a].push(b);
        edges[b].push(a);
    }
    dfs(&edges, &mut de, 0, 1 << 60);

    let mut pushed = vec![0; N];
    for (n, cnt) in px {
        pushed[n] += cnt;
    }
    let mut counts = vec![0; N];
    dfs2(&mut counts, &de, &pushed, 0, 0);
    println!("{}", counts.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

fn dfs(edges: &Vec<Vec<usize>>, de: &mut Vec<Vec<usize>>, cur: usize, from: usize) {
    for &to in &edges[cur] {
        if to == from {
            continue;
        }
        de[cur].push(to);
        dfs(edges, de, to, cur);
    }
}

fn dfs2(counts: &mut Vec<i64>, de: &Vec<Vec<usize>>, pushed: &Vec<i64>, cur: usize, acc: i64) {
    let p = pushed[cur] + acc;
    counts[cur] = p;
    for &child in &de[cur] {
        dfs2(counts, de, pushed, child, p);
    }
}
