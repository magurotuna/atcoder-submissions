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

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        AB: [(usize1, usize1); M],
        CD: [(usize1, usize1); K],
    }

    let mut relations = vec![(HashSet::new(), HashSet::new()); N];

    let mut ds = DisjointSet::new(N);

    for i in 0..M {
        let (a, b) = AB[i];
        relations[a].0.insert(b);
        relations[b].0.insert(a);
        ds.unite(a, b);
    }
    for i in 0..K {
        let (c, d) = CD[i];
        relations[c].1.insert(d);
        relations[d].1.insert(c);
    }

    let mut ans = Vec::with_capacity(N);
    for i in 0..N {
        let (root, count) = ds.root(i).unwrap();
        let mut count = dbg!(count - 1);
        count -= dbg!(relations[i].0.len());
        for &block in relations[i].1.iter() {
            dbg!((block, i));
            if ds.same(i, block) {
                count -= 1;
            }
        }
        ans.push(count.to_string());
    }
    println!("{}", ans.join(" "));
}

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    count: Vec<usize>,
    rank: Vec<usize>,
}
impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: vec![1; n],
            rank: vec![0; n],
        }
    }
    pub fn root(&mut self, x: usize) -> Option<(usize, usize)> {
        if x > self.parent.len() {
            None
        } else if self.parent[x] == x {
            Some((x, self.count[x]))
        } else {
            let px = self.parent[x];
            let (root, root_count) = self.root(px).unwrap();
            self.parent[x] = root;
            Some((root, root_count))
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let (x_root, x_root_count) = match self.root(x) {
            None => return,
            Some(val) => val,
        };
        let (y_root, y_root_count) = match self.root(y) {
            None => return,
            Some(val) => val,
        };
        if x_root == y_root {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x_root] = y_root;
            self.count[x_root] = x_root_count + y_root_count;
            self.count[y_root] = x_root_count + y_root_count;
        } else {
            self.parent[y_root] = x_root;
            self.count[y_root] = y_root_count + x_root_count;
            self.count[x_root] = x_root_count + y_root_count;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let (x_root, _) = match self.root(x) {
            None => return false,
            Some(val) => val,
        };
        let (y_root, _) = match self.root(y) {
            None => return false,
            Some(val) => val,
        };
        x_root == y_root
    }
}
