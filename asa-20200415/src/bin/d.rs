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
        Q: usize,
        lr: [(usize, usize); Q],
    }
    use std::collections::HashSet;
    let primes: HashSet<usize> = 110000.lower_primes().into_iter().collect();
    let mut acc = Vec::with_capacity(110000);
    acc.push(0);
    for i in 0..100010 {
        let &last = acc.last().unwrap();
        if i % 2 == 0 {
            acc.push(last);
        } else {
            if primes.contains(&i) && primes.contains(&((i+1) / 2)) {
                acc.push(last + 1);
            } else {
                acc.push(last);
            }
        }
    }

    let mut ans = Vec::with_capacity(Q);
    for (l, r) in lr {
        let cnt = format!("{}", acc[r+1] - acc[l]);
        ans.push(cnt);
    }
    println!("{}", ans.join("\n"));
}

pub trait Prime {
    fn lower_primes(&self) -> Vec<usize>;
    fn factorize(&self) -> std::collections::HashMap<usize, usize>;
}
impl Prime for usize {
    #[doc = " エラトステネスの篩を用いてself以下の素数を求める"]
    #[doc = " 計算量: O(n log log n)"]
    fn lower_primes(&self) -> Vec<usize> {
        let &this = self;
        let mut v = Vec::new();
        if this < 2 {
            return v;
        }
        let mut deque = (2..(this + 1)).collect::<std::collections::VecDeque<usize>>();
        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p as f64 <= (this as f64).sqrt() {
            deque = deque.iter().filter(|&&x| x % p != 0).map(|x| *x).collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }
    #[doc = " エラトステネスの篩を用いてselfを素因数分解する"]
    fn factorize(&self) -> std::collections::HashMap<usize, usize> {
        let mut ret = std::collections::HashMap::new();
        let primes = ((*self as f64).sqrt() as usize).lower_primes();
        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == 0 {
                tmp /= prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > 1 {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}
