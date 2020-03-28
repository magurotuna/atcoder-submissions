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
        X: usize,
        Y: usize,
        A: usize,
        B: usize,
        C: usize,
        p: [i64; A],
        q: [i64; B],
        r: [i64; C],
    }
    let mut p = p;
    let mut q = q;
    let mut r = r;
    p.sort_unstable_by(|a, b| b.cmp(&a));
    q.sort_unstable_by(|a, b| b.cmp(&a));
    r.sort_unstable_by(|a, b| b.cmp(&a));

    let mut acc_p = vec![0; A + 1];
    let mut acc_q = vec![0; B + 1];
    for i in 0..A {
        acc_p[i + 1] = acc_p[i] + p[i];
    }
    for i in 0..B {
        acc_q[i + 1] = acc_q[i] + q[i];
    }

    let mut dp = vec![0; C + 1];
    dp[0] = acc_p[X] + acc_q[Y];

    let mut ans = dp[0];
    let mut cur_x = X - 1;
    let mut cur_y = Y - 1;
    for i in 1..=C {
        if cur_x != 1 << 60 && cur_y != 1 << 60 {
            if p[cur_x] >= q[cur_y] && r[i - 1] > q[cur_y] {
                dp[i] = dp[i - 1] - q[cur_y] + r[i - 1];
                ans = dp[i];
                if cur_y > 0 {
                    cur_y -= 1;
                } else {
                    cur_y = 1 << 60;
                }
            } else if q[cur_y] >= p[cur_x] && r[i - 1] > p[cur_x] {
                dp[i] = dp[i - 1] - p[cur_x] + r[i - 1];
                ans = dp[i];
                if cur_x > 0 {
                    cur_x -= 1;
                } else {
                    cur_x = 1 << 60;
                }
            } else {
                break;
            }
        } else if cur_x == 1 << 60 && cur_y == 1 << 60 {
            break;
        } else if cur_x == 1 << 60 {
            if q[cur_y] < r[i - 1] {
                dp[i] = dp[i - 1] - q[cur_y] + r[i - 1];
                ans = dp[i];
                if cur_y > 0 {
                    cur_y -= 1;
                } else {
                    cur_y = 1 << 60;
                }
            } else {
                break;
            }
        } else if cur_y == 1 << 60 {
            if p[cur_x] < r[i - 1] {
                dp[i] = dp[i - 1] - p[cur_x] + r[i - 1];
                ans = dp[i];
                if cur_x > 0 {
                    cur_x -= 1;
                } else {
                    cur_x = 1 << 60;
                }
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
