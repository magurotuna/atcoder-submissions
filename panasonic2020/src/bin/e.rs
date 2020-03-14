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

use std::cmp::min;
fn main() {
    input! {
        a: chars,
        b: chars,
        c: chars,
    }
    let al = a.len() as i64;
    let bl = b.len() as i64;
    let cl = c.len() as i64;

    let total = al + bl + cl;

    let ab = calc(&a, &b);
    let ac = calc(&a, &c);
    let ba = calc(&b, &a);
    let bc = calc(&b, &c);
    let ca = calc(&c, &a);
    let cb = calc(&c, &b);

    let mut ans: i64 = 1 << 60;
    // abc
    ans = min(ans, total - ab - bc);
    // acb
    ans = min(ans, total - ac - cb);
    // bac
    ans = min(ans, total - ba - ac);
    // bca
    ans = min(ans, total - bc - ca);
    // cab
    ans = min(ans, total - ca - ab);
    // cba
    ans = min(ans, total - cb - ba);

    println!("{}", ans);
}

fn calc(x: &[char], y: &[char]) -> i64 {
    let mut intersection = 0;
    for i in 1..=min(x.len(), y.len()) {
        let mut ok = true;
        for j in 0..i {
            let cx = x[x.len() - i + j];
            let cy = y[j];
            if cx != '?' && cy != '?' && cx != cy {
                ok = false;
                break;
            }
        }
        if ok {
            intersection = i;
        }
    }
    intersection as i64
}
