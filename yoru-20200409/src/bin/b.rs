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
        A: i32,
        B: i32,
        C: i32,
    }
    use std::collections::{VecDeque, HashSet};
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((A, B, C, 0));

    while let Some((a, b, c, cnt)) = queue.pop_front() {
        if a == b && b == c {
            println!("{}", cnt);
            return;
        }

        if visited.contains(&(a, b, c)) {
            continue;
        }
        visited.insert((a, b, c));
        let nc = cnt + 1;
        queue.push_back((a + 1, b + 1, c, nc));
        queue.push_back((a + 1, b, c + 1, nc));
        queue.push_back((a, b + 1, c + 1, nc));
        queue.push_back((a + 2, b, c, nc));
        queue.push_back((a, b + 2, c, nc));
        queue.push_back((a, b, c + 2, nc));
    }
    unreachable!();
}
