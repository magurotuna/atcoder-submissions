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
        M: usize,
        AB: [(usize1, usize1); M],
    }
    let rev_ab = AB.iter().copied().rev().collect::<Vec<_>>();
    let mut inconvenience = calc(N);
    let mut ds = DisjointSet::new(N);

    use std::collections::VecDeque;
    let mut ans = VecDeque::new();
    for i in 0..M {
        ans.push_front(inconvenience.to_string());
        if ds.same(rev_ab[i].0, rev_ab[i].1) {
            continue;
        }
        let s1 = ds.size(rev_ab[i].0).unwrap();
        let s2 = ds.size(rev_ab[i].1).unwrap();
        ds.unite(rev_ab[i].0, rev_ab[i].1);
        inconvenience -= s1 * s2;
    }

    println!("{}", ans.into_iter().collect::<Vec<_>>().join("\n"));
}

fn calc(n: usize) -> i64 {
    ((n * (n - 1)) / 2) as i64
}

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    sizes: Vec<i64>,
    rank: Vec<i64>,
}
impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sizes: vec![1; n],
            rank: vec![0; n],
        }
    }
    pub fn root(&mut self, x: usize) -> Option<usize> {
        if x >= self.parent.len() {
            None
        } else if self.parent[x] == x {
            Some(x)
        } else {
            let px = self.parent[x];
            let root = self.root(px).unwrap();
            self.parent[x] = root;
            Some(root)
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = match self.root(x) {
            None => return,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return,
            Some(val) => val,
        };
        if x_root == y_root {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x_root] = y_root;
            self.sizes[y_root] += self.sizes[x_root];
        } else {
            self.parent[y_root] = x_root;
            self.sizes[x_root] += self.sizes[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = match self.root(x) {
            None => return false,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return false,
            Some(val) => val,
        };
        x_root == y_root
    }
    pub fn size(&mut self, x: usize) -> Option<i64> {
        self.root(x).map(|r| self.sizes[r])
    }
}
