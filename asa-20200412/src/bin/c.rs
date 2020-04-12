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
        N: usize,
        A: [i64; N],
    }

    if N == 1 {
        println!("1");
        return;
    }

    let mut ans = 1;
    let mut is_growing = None;
    let mut prev = A[0];

    for i in 1..N {
        let v = A[i];
        match is_growing {
            None => {
                if prev > v {
                    is_growing = Some(false);
                    prev = v;
                } else if prev < v {
                    is_growing = Some(true);
                    prev = v;
                }
            }
            Some(true) => {
                if prev > v {
                    ans += 1;
                    prev = v;
                    is_growing = None;
                } else {
                    prev = v;
                }
            }
            Some(false) => {
                if prev < v {
                    ans += 1;
                    prev = v;
                    is_growing = None;
                } else {
                    prev = v;
                }
            }
        }
    }

    println!("{}", ans);
}
