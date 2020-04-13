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
        K: usize,
        A: [usize; N],
    }
    if K == 1 {
        println!("{}", A.into_iter().filter(|&x| x == 1).count());
        return;
    }
    //if N == 1 {
        //let ans = if A[0] % K == 1 { 1 } else { 0 };
        //println!("{}", ans);
        //return;
    //}
    let mut cumsum = Vec::with_capacity(N);
    cumsum.push(0);
    for i in 0..N {
        let prev = *cumsum.last().unwrap();
        cumsum.push(prev + A[i]);
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = 0usize;
    for r in (1..(N + 1)).rev() {
        let rhs = (cumsum[r] - r) % K;
        if map.is_empty() {
            for l in ((r + 1).saturating_sub(K)..r).rev() {
                let lhs = (cumsum[l] - l) % K;
                *map.entry(lhs).or_insert(0) += 1;
            }
        } else {
            if r + 1 >= K {
                let l = r + 1 - K;
                let lhs = (cumsum[l] - l) % K;
                *map.entry(lhs).or_insert(0) += 1;
            }
        }
        if let Some(&l_cnt) = map.get(&rhs) {
            ans += l_cnt;
        }

        let tmp = (cumsum[r - 1] - (r - 1)) % K;
        *map.get_mut(&tmp).unwrap() -= 1;
    }
    println!("{}", ans);
}
