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

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
struct MyFloat(f64);

impl Eq for MyFloat {}

impl PartialOrd for MyFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let &MyFloat(ref inner1) = self;
        let &MyFloat(ref inner2) = other;
        inner1.partial_cmp(inner2)
    }
}

impl Ord for MyFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        let &MyFloat(ref inner1) = self;
        let &MyFloat(ref inner2) = other;
        inner1.partial_cmp(inner2).unwrap()
    }
}

fn main() {
    input! {
        A: i64,
        B: i64,
        C: i64,
        D: i64,
        E: i64,
        F: i64,
    }

    let mut memo = HashMap::new();

    let (mw, ms) = {
        let mut w = A * 100;
        let mut s = 0;
        let (ww, ss) = dfs(0, 0, 100 * A, 0, F, 100 * A, 100 * B, C, D, E, &mut memo);
        if density(w, s) < density(ww, ss) {
            w = ww;
            s = ss;
        }
        let (ww, ss) = dfs(0, 0, 100 * B, 0, F, 100 * A, 100 * B, C, D, E, &mut memo);
        if density(w, s) < density(ww, ss) {
            w = ww;
            s = ss;
        }
        (w, s)
    };
    println!("{} {}", mw + ms, ms);
}

fn dfs(
    cw: i64,
    cs: i64,
    nw: i64,
    ns: i64,
    limit: i64,
    m1: i64,
    m2: i64,
    m3: i64,
    m4: i64,
    e: i64,
    memo: &mut HashMap<(i64, i64, i64, i64), (i64, i64)>,
) -> (i64, i64) {
    if memo.contains_key(&(cw, cs, nw, ns)) {
        return *memo.get(&(cw, cs, nw, ns)).unwrap();
    }

    if cw + nw + cs + ns > limit {
        memo.insert((cw, cs, nw, ns), (cw, cs));
        return (cw, cs);
    }

    let cw = cw + nw;
    let cs = cs + ns;

    let mut w = cw;
    let mut s = cs;
    let (ww, ss) = dfs(cw, cs, m1, 0, limit, m1, m2, m3, m4, e, memo);
    if density(w, s) < density(ww, ss) {
        w = ww;
        s = ss;
    }
    let (ww, ss) = dfs(cw, cs, m2, 0, limit, m1, m2, m3, m4, e, memo);
    if density(w, s) < density(ww, ss) {
        w = ww;
        s = ss;
    }
    if (cw * e / 100) >= cs + m3 {
        let (ww, ss) = dfs(cw, cs, 0, m3, limit, m1, m2, m3, m4, e, memo);
        if density(w, s) < density(ww, ss) {
            w = ww;
            s = ss;
        }
    }
    if (cw * e / 100) >= cs + m4 {
        let (ww, ss) = dfs(cw, cs, 0, m4, limit, m1, m2, m3, m4, e, memo);
        if density(w, s) < density(ww, ss) {
            w = ww;
            s = ss;
        }
    }
    memo.insert((cw, cs, nw, ns), (w, s));
    (w, s)
}

fn density(w: i64, s: i64) -> MyFloat {
    MyFloat(s as f64 / (w + s) as f64)
}
