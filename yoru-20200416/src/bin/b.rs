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
        H: usize,
        W: usize,
        S: [chars; H],
    }
    let mut map = vec![vec!['#'; W]; H];

    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                map[h][w] = '#';
                continue;
            }
            let mut bombs = 0;
            if h > 0 && S[h - 1][w] == '#' {
                bombs += 1;
            }
            if h + 1 < H && S[h + 1][w] == '#' {
                bombs += 1;
            }
            if w > 0 && S[h][w - 1] == '#' {
                bombs += 1;
            }
            if w + 1 < W && S[h][w + 1] == '#' {
                bombs += 1;
            }
            if h > 0 && w > 0 && S[h - 1][w - 1] == '#' {
                bombs += 1;
            }
            if h > 0 && w + 1 < W && S[h - 1][w + 1] == '#' {
                bombs += 1;
            }
            if h + 1 < H && w > 0 && S[h + 1][w - 1] == '#' {
                bombs += 1;
            }
            if h + 1 < H && w + 1 < W && S[h + 1][w + 1] == '#' {
                bombs += 1;
            }
            map[h][w] = std::char::from_digit(bombs, 10).unwrap();
        }
    }
    for h in 0..H {
        let mut s = String::new();
        for w in 0..W {
            s.push(map[h][w]);
        }
        println!("{}", s);
    }
}
