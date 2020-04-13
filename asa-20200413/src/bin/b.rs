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

use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        H: usize,
        W: usize,
        s: [chars; H],
    }

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    for h in 0..H {
        for w in 0..W {
            if visited.contains(&(h, w)) {
                continue;
            }

            if s[h][w] == '.' {
                continue;
            }

            q.clear();
            q.push_back((h, w));

            let mut neighbor_painted = false;

            while let Some((h, w)) = q.pop_front() {
                if visited.contains(&(h, w)) {
                    continue;
                }
                visited.insert((h, w));

                // up
                if h > 0 && s[h - 1][w] == '#' {
                    q.push_back((h - 1, w));
                    neighbor_painted = true;
                }
                // down
                if h + 1 < H && s[h + 1][w] == '#' {
                    q.push_back((h + 1, w));
                    neighbor_painted = true;
                }
                // left
                if w > 0 && s[h][w - 1] == '#' {
                    q.push_back((h, w - 1));
                    neighbor_painted = true;
                }
                // right
                if w + 1 < W && s[h][w + 1] == '#' {
                    q.push_back((h, w + 1));
                    neighbor_painted = true;
                }
            }

            if !neighbor_painted {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
