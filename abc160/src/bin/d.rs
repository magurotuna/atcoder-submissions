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
        X: usize1,
        Y: usize1,
    }

    let mut adj = Vec::with_capacity(N);
    for i in 0..N {
        adj.push(vec![]);
    }
    for i in 0..N {
        if i == N - 1 {
            ()
        } else {
            adj[i].push(i + 1);
            adj[i + 1].push(i);
        }

        if i == X {
            adj[i].push(Y);
        }
        if i == Y {
            adj[i].push(X);
        }
    }
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut distances = vec![0; N];

    for s in 0..N {
        // 始点sから各頂点への最短距離
        let mut d = vec![1 << 60; N];
        d[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, s)));

        while !heap.is_empty() {
            let Reverse((dist, v)) = heap.pop().unwrap();
            if d[v] < dist {
                continue;
            }

            for i in 0..adj[v].len() {
                let to = adj[v][i];
                if d[to] > d[v] + 1 {
                    d[to] = d[v] + 1;
                    heap.push(Reverse((d[to], to)));
                }
            }
        }

        for i in 0..N {
            distances[d[i]] += 1;
        }
    }

    println!(
        "{}",
        distances
            .into_iter()
            .skip(1)
            .map(|d| (d / 2).to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
