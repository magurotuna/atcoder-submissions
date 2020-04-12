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

use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
        S: chars,
    }
    let mut r_set = Vec::new();
    let mut g_set = Vec::new();
    let mut b_set = Vec::new();
    let mut r_set2 = HashSet::new();
    let mut g_set2 = HashSet::new();
    let mut b_set2 = HashSet::new();
    for i in 0..N {
        if S[i] == 'R' {
            r_set.push(i);
            r_set2.insert(i);
        } else if S[i] == 'G' {
            g_set.push(i);
            g_set2.insert(i);
        } else {
            b_set.push(i);
            b_set2.insert(i);
        }
    }
    let mut ans = 0 as i64;
    for i in 0..(N - 2) {
        for j in (i + 1)..(N - 1) {
            if S[i] == S[j] {
                continue;
            }
            let c = last_ch(S[i], S[j]);
            let l = match c {
                'R' => r_set.len() as i64,
                'G' => g_set.len() as i64,
                'B' => b_set.len() as i64,
                _ => unreachable!(),
            };
            let lower = {
                let mut ok = l;
                let mut ng = -1 as i64;
                while ok - ng > 1 {
                    let mid = (ok + ng) / 2;
                    if c == 'R' {
                        if r_set[mid as usize] >= j + 1 {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    } else if c == 'G' {
                        if g_set[mid as usize] >= j + 1 {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    } else {
                        if b_set[mid as usize] >= j + 1 {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    }
                }
                ok
            };
            ans += l - lower;
            if c == 'R' && r_set2.contains(&(2 * j - i)) {
                ans -= 1;
            }
            if c == 'G' && g_set2.contains(&(2 * j - i)) {
                ans -= 1;
            }
            if c == 'B' && b_set2.contains(&(2 * j - i)) {
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}

fn last_ch(a: char, b: char) -> char {
    *['R', 'G', 'B'].iter().find(|&&x| x != a && x != b).unwrap()
}
