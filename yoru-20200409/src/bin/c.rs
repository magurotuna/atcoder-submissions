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
        S: chars,
    }
    let mut rl = Vec::new();
    let mut r = 0;
    let mut l = 0;
    for i in 0..S.len() {
        if S[i] == 'R' && l > 0 {
            rl.push((r, l));
            r = 1;
            l = 0;
        } else if S[i] == 'R' {
            r += 1;
        } else {
            l += 1;
        }
    }
    rl.push((r, l));
    use std::cmp::max;
    let mut ans = vec![0; S.len()];
    let mut cur = 0;
    for (r, l) in rl {
        if (r + l) % 2 == 0 {
            let t = (r + l) / 2;
            ans[cur + r - 1] = t;
            ans[cur + r] = t;
        } else {
            let balance = max(r, l) - 1;
            let big = (r + l + 1) / 2;
            let small = (r + l - 1) / 2;
            if balance % 2 == 0 {
                if r > l {
                    ans[cur + r - 1] = big;
                    ans[cur + r] = small;
                } else {
                    ans[cur + r - 1] = small;
                    ans[cur + r] = big;
                }
            } else {
                if r > l {
                    ans[cur + r - 1] = small;
                    ans[cur + r] = big;
                } else {
                    ans[cur + r - 1] = big;
                    ans[cur + r] = small;
                }
            }
        }
        cur += r + l;
    }
    println!("{}", ans.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
