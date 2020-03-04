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
        A: usize,
        B: usize,
        Q: usize,
        s: [i64; A],
        t: [i64; B],
        x: [i64; Q],
    }

    let inf = 1 << 60;

    use std::cmp::min;
    use std::collections::BTreeSet;
    let mut s_set = BTreeSet::new();
    let mut t_set = BTreeSet::new();

    for ss in s {
        s_set.insert(ss);
    }
    for tt in t {
        t_set.insert(tt);
    }

    let mut ans = Vec::new();
    for q in x {
        let &lower_s = s_set.range(..q).last().unwrap_or(&-inf);
        let &upper_s = s_set.range(q..).next().unwrap_or(&inf);
        let &lower_t = t_set.range(..q).last().unwrap_or(&-inf);
        let &upper_t = t_set.range(q..).next().unwrap_or(&inf);
        let mut min_d = 1 << 60;
        min_d = min(min_d, dist(q, lower_s, lower_t));
        min_d = min(min_d, dist(q, lower_s, upper_t));
        min_d = min(min_d, dist(q, upper_s, lower_t));
        min_d = min(min_d, dist(q, upper_s, upper_t));
        ans.push(min_d.to_string());
    }

    println!("{}", ans.join("\n"));
}

fn dist(x: i64, s: i64, t: i64) -> i64 {
    use std::cmp::{max, min};
    if x <= min(s, t) {
        (max(s, t) - x).abs()
    } else if max(s, t) <= x {
        (x - min(s, t)).abs()
    } else {
        let ds = (x - s).abs();
        let dt = (x - t).abs();
        min(ds, dt) * 2 + max(ds, dt)
    }
}
