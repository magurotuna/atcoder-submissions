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
        X: usize,
    }
    let mut height = vec![None; N+1];
    let mut num_p = vec![None; N+1];
    num_p[0] = Some(1);
    height[0] = Some(1);
    calc(&mut height, &mut num_p, N);

    let height = height.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    let num_p = num_p.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut ans = 0;
    let mut rest_x = X - 1; // eat bun at the bottom
    let mut cur_level = N - 1;
    while rest_x > 0 {
        if rest_x == height[cur_level] {
            ans += num_p[cur_level];
            rest_x -= height[cur_level];
        } else if rest_x == height[cur_level] + 1 {
            ans += 1 + num_p[cur_level];
            rest_x -= 1 + height[cur_level];
        } else if rest_x == 2 * height[cur_level] + 1 {
            ans += 1 + 2 * num_p[cur_level];
            rest_x -= 1 + 2 * height[cur_level];
        } else if rest_x == 2 * height[cur_level] + 2 {
            ans += 1 + 2 * num_p[cur_level];
            rest_x -= 2 + 2 * height[cur_level];
        } else if rest_x < height[cur_level] {
            // inside the bottom L(N-1) burger
            cur_level -= 1;
            rest_x -= 1;
        } else {
            // inside the upper L(N-1) burger
            ans += 1 + num_p[cur_level];
            rest_x -= 2 + height[cur_level];
            cur_level -= 1;
        }
    }
    println!("{}", ans);
}

fn calc(height: &mut Vec<Option<usize>>, num_p: &mut Vec<Option<usize>>, level: usize) -> (usize, usize) {
    if height[level].is_some() && num_p[level].is_some() {
        return (height[level].unwrap(), num_p[level].unwrap());
    }

    let (h, n) = calc(height, num_p, level - 1);
    let next_h = 2 * h + 3;
    let next_n = 2 * n + 1;
    height[level] = Some(next_h);
    num_p[level] = Some(next_n);
    (next_h, next_n)
}
