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
    // solution_binary_search();
    // solution_priority_queue();
    solution_well_sort();
}

fn solution_well_sort() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [i64; X],
        B: [i64; Y],
        C: [i64; Z],
    }
    let mut A = A;
    let mut B = B;
    let mut C = C;

    let mut D = Vec::with_capacity(X * Y);
    for i in 0..X {
        for j in 0..Y {
            D.push(A[i] + B[j]);
        }
    }
    D.sort_by(|a, b| b.cmp(a));

    let mut E = Vec::with_capacity(K * K);
    for i in 0..std::cmp::min(K, X * Y) {
        for j in 0..std::cmp::min(K, Z) {
            E.push(D[i] + C[j]);
        }
    }
    E.sort_by(|a, b| b.cmp(a));

    println!(
        "{}",
        E.into_iter()
            .take(K)
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solution_priority_queue() {
    use std::collections::{BinaryHeap, HashSet};
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [i64; X],
        B: [i64; Y],
        C: [i64; Z],
    }
    let mut A = A;
    let mut B = B;
    let mut C = C;
    A.sort_by(|a, b| b.cmp(a));
    B.sort_by(|a, b| b.cmp(a));
    C.sort_by(|a, b| b.cmp(a));
    // let A: VecDeque<_> = A.into();
    // let B: VecDeque<_> = B.into();
    // let C: VecDeque<_> = C.into();

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
    struct Point {
        point: i64,
        a: usize,
        b: usize,
        c: usize,
    }

    impl Ord for Point {
        fn cmp(&self, other: &Point) -> std::cmp::Ordering {
            self.point.cmp(&other.point)
        }
    }

    let mut bh = BinaryHeap::new();
    bh.push(Point {
        point: A[0] + B[0] + C[0],
        a: 0,
        b: 0,
        c: 0,
    });

    let mut used = HashSet::new();
    used.insert((0, 0, 0));

    for _ in 0..K {
        let p = bh.pop().unwrap();
        println!("{}", p.point);

        let a = p.a;
        let b = p.b;
        let c = p.c;

        if !used.contains(&(a + 1, b, c)) && a + 1 < X {
            used.insert((a + 1, b, c));
            bh.push(Point {
                point: A[a + 1] + B[b] + C[c],
                a: a + 1,
                b,
                c,
            });
        }
        if !used.contains(&(a, b + 1, c)) && b + 1 < Y {
            used.insert((a, b + 1, c));
            bh.push(Point {
                point: A[a] + B[b + 1] + C[c],
                a,
                b: b + 1,
                c,
            });
        }
        if !used.contains(&(a, b, c + 1)) && c + 1 < Z {
            used.insert((a, b, c + 1));
            bh.push(Point {
                point: A[a] + B[b] + C[c + 1],
                a,
                b,
                c: c + 1,
            });
        }
    }
}

fn solution_binary_search() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [i64; X],
        B: [i64; Y],
        C: [i64; Z],
    }
    let mut A = A;
    let mut B = B;
    let mut C = C;
    A.sort_by(|a, b| b.cmp(a));
    B.sort_by(|a, b| b.cmp(a));
    C.sort_by(|a, b| b.cmp(a));
    // おいしさの合計がp以上であるような組み合わせの個数がk個以上あるかどうかを判定する
    let solve = |p: i64, vec: &mut Vec<i64>| {
        let mut cnt = 0;
        for i in 0..X {
            for j in 0..Y {
                for k in 0..Z {
                    let s = A[i] + B[j] + C[k];
                    if s < p {
                        break;
                    }
                    cnt += 1;
                    vec.push(s);
                    if cnt >= K {
                        return true;
                    }
                }
            }
        }
        false
    };

    let mut ok = 0i64;
    let mut ng: i64 = 1 << 60;
    let mut sums = Vec::new();
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if solve(mid, &mut sums) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    // dbg!(ok, ng);

    sums.clear();
    solve(ok + 1, &mut sums);
    sums.sort_by(|a, b| b.cmp(a));

    println!(
        "{}",
        sums.into_iter()
            .chain(vec![ok].into_iter().cycle())
            .take(K)
            .map(|s| format!("{}", s))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
