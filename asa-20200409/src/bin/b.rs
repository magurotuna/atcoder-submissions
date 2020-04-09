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

// TODO: WAなのでときなおす！！！
fn main() {
    input! {
        S: chars,
    }
    let N = S.len() + 1;
    let mut r_arrows = vec![('>', 0); N];
    let mut l_arrows = vec![('<', 0); N];

    for i in 0..S.len() {
        let (ar, cnt) = l_arrows[i];
        if ar == S[i] {
            l_arrows[i + 1] = (ar, cnt + 1);
        } else {
            let next_ar = if ar == '<' { '>' } else { '<' };
            l_arrows[i + 1] = (next_ar, 1);
        }
    }
    for i in (0..S.len()).rev() {
        let (ar, cnt) = r_arrows[i + 1];
        if ar == S[i] {
            r_arrows[i] = (ar, cnt + 1);
        } else {
            let next_ar = if ar == '<' { '>' } else { '<' };
            r_arrows[i] = (next_ar, 1);
        }
    }
    let mut ans = 0;
    let mut nums = Vec::new();
    use std::cmp::max;
    for i in 0..N {
        let (ra, rcnt) = r_arrows[i];
        let (la, lcnt) = l_arrows[i];
        if ra == la {
            if ra == '<' {
                nums.push(lcnt);
                ans += lcnt;
            } else {
                nums.push(rcnt);
                ans += rcnt;
            }
        } else if la == '>' && ra == '<' {
            nums.push(0);
            continue;
        } else {
            ans += max(rcnt, lcnt);
            nums.push(max(rcnt, lcnt));
        }
    }
    println!("{}", ans);
}
