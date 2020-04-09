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
        P: i32,
        A: [i32; N],
    }
    let mut num_even = 0;
    let mut num_odd = 0;
    for i in 0..N {
        if A[i] % 2 == 0 {
            num_even += 1;
        } else {
            num_odd += 1;
        }
    }

    let comb = Comb::new(100, 1_000_000_007);

    let even_select = 2usize.pow(num_even as u32);

    let mut ans = 0;
    if P == 0 {
        // 奇数の袋を偶数個選べば P == 0 になる
        let mut odd_cnt = 0;
        while odd_cnt <= num_odd {
            // 奇数の袋を odd_cnt 個選ぶときの袋の選び方
            ans += comb.calc(num_odd, odd_cnt) * even_select;
            odd_cnt += 2;
        }
    } else {
        // 奇数の袋を奇数個選べば P == 1 になる
        let mut odd_cnt = 1;
        while odd_cnt <= num_odd {
            ans += comb.calc(num_odd, odd_cnt) * even_select;
            odd_cnt += 2;
        }
    }
    println!("{}", ans);
}

#[doc = " 二項係数を mod のもとで求める"]
#[doc = " cf. [よくやる二項係数 (nCk mod. p)、逆元 (a^-1 mod. p) の求め方 - けんちょんの競プロ精進記録](http://drken1215.hatenablog.com/entry/2018/06/08/210000)"]
pub struct Comb {
    max_size: usize,
    modulo: usize,
    factorical_table: Vec<usize>,
    factorical_inverse_table: Vec<usize>,
    inverse_table: Vec<usize>,
}
impl Comb {
    pub fn new(max_size: usize, modulo: usize) -> Self {
        let max_size = std::cmp::max(10, max_size);
        assert!(max_size <= 10_000_000);
        let mut factorical_table = vec![0; max_size];
        let mut factorical_inverse_table = vec![0; max_size];
        let mut inverse_table = vec![0; max_size];
        factorical_table[0] = 1;
        factorical_table[1] = 1;
        factorical_inverse_table[0] = 1;
        factorical_inverse_table[1] = 1;
        inverse_table[1] = 1;
        for i in 2..max_size {
            factorical_table[i] = factorical_table[i - 1] * i % modulo;
            inverse_table[i] = modulo - inverse_table[modulo % i] * (modulo / i) % modulo;
            factorical_inverse_table[i] =
                factorical_inverse_table[i - 1] * inverse_table[i] % modulo;
        }
        Comb {
            max_size: max_size,
            modulo: modulo,
            factorical_table: factorical_table,
            factorical_inverse_table: factorical_inverse_table,
            inverse_table: inverse_table,
        }
    }
    pub fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.factorical_table[n]
                * (self.factorical_inverse_table[k] * self.factorical_inverse_table[n - k]
                    % self.modulo)
                % self.modulo
        }
    }
}
