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

use std::collections::HashMap;
fn main() {
    input! {
        K: usize,
    }
    let mut ans = 0;
    //let mut memo = HashMap::new();
    for a in 1..=K {
        for b in 1..=K {
            for c in 1..=K {
                ans += gcd(a as u64, gcd(b as u64, c as u64));
                //let (a, b, c) = {
                //let mut slice = [a, b, c];
                //slice.sort();
                //(slice[0] as u64, slice[1] as u64, slice[2] as u64)
                //};
                //ans += gcd3(a, b, c, &mut memo);
            }
        }
    }
    println!("{}", ans);
}

fn gcd3(a: u64, b: u64, c: u64, memo: &mut HashMap<(u64, u64, u64), u64>) -> u64 {
    if memo.contains_key(&(a, b, c)) {
        return *memo.get(&(a, b, c)).unwrap();
    }

    let ans = gcd(a, gcd(b, c));
    memo.insert((a, b, c), ans);
    ans
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
