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
        H: usize,
        W: usize,
        s: [chars; H],
    }
    use std::collections::{VecDeque, HashSet};
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0, 1));

    let mut black = 0;
    for i in 0..H {
        for j in 0..W {
            if s[i][j] == '#' {
                black += 1;
            }
        }
    }

    while let Some((h, w, cnt)) = queue.pop_front() {
        if visited.contains(&(h, w)) {
            continue;
        }
        if h == H - 1 && w == W - 1 {
            println!("{}", H * W - black - cnt);
            return;
        }

        visited.insert((h, w));
        // up
        if h >= 1 && s[h-1][w] == '.' {
            queue.push_back((h-1, w, cnt+1));
        }
        // down
        if h + 1 < H && s[h+1][w] == '.' {
            queue.push_back((h+1, w, cnt+1));
        }
        // left
        if w >= 1 && s[h][w-1] == '.' {
            queue.push_back((h, w-1, cnt+1));
        }
        // right
        if w + 1 < W && s[h][w+1] == '.' {
            queue.push_back((h, w+1, cnt+1));
        }
    }
    println!("-1");
}
