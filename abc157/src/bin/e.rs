#![allow(unused_macros)]
use std::collections::HashSet;

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

// アルファベット小文字を受け取って数値に変換して返す a -> 0, b -> 1, ..., z -> 26 のような対応
fn alpha_to_pos(alphabet: char) -> usize {
    alphabet as usize - 'a' as usize
}

fn main() {
    input! {
        N: usize,
        S: chars,
        Q: usize,
        qs: [(usize, usize, String); Q],
    }
    // セグ木で殴る（区間に対して、使われているアルファベット集合の和集合をとっていく）
    let mut st = SegmentTree::new(0usize, N, |x, y| x | y);
    for i in 0..N {
        st.update(i, 1 << alpha_to_pos(S[i]));
    }

    let mut ans = Vec::with_capacity(Q);
    for q in qs {
        if q.0 == 1 {
            let c = q.2.chars().next().unwrap();
            st.update(q.1 - 1, 1 << alpha_to_pos(c));
        } else {
            let left = q.1 - 1;
            let right = q.2.parse::<usize>().unwrap();
            ans.push(st.query(left, right).count_ones().to_string());
        }
    }
    println!("{}", ans.join("\n"));
}

#[derive(Debug)]
pub struct SegmentTree<T, F>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    values: Vec<T>,
    n_leaves: usize,
    identity_elem: T,
    func: F,
}
impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    pub fn new(init_value: T, size: usize, func: F) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            values: vec![init_value.clone(); 2 * n - 1],
            n_leaves: n,
            identity_elem: init_value.clone(),
            func,
        }
    }
    pub fn update(&mut self, leaf_number: usize, replace_value: T) {
        let mut node_number = leaf_number + self.n_leaves - 1;
        self.values[node_number] = replace_value;
        while node_number > 0 {
            node_number = (node_number - 1) / 2;
            self.values[node_number] = (self.func)(
                self.values[node_number * 2 + 1].clone(),
                self.values[node_number * 2 + 2].clone(),
            );
        }
    }
    pub fn query(&self, begin: usize, end: usize) -> T {
        self.internal_query(begin, end, 0, 0, self.n_leaves)
    }
    fn internal_query(
        &self,
        begin: usize,
        end: usize,
        node_number: usize,
        left: usize,
        right: usize,
    ) -> T {
        if right <= begin || end <= left {
            self.identity_elem.clone()
        } else if begin <= left && right <= end {
            self.values[node_number].clone()
        } else {
            let c1 = self.internal_query(begin, end, 2 * node_number + 1, left, (left + right) / 2);
            let c2 =
                self.internal_query(begin, end, 2 * node_number + 2, (left + right) / 2, right);
            (self.func)(c1, c2)
        }
    }
}
