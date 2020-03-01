use libprocon::*;

// this approach seems right, but needs to be less computation
fn this_is_tle() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N],
    }
    let mut A = A;
    A.sort_by(|a, b| b.cmp(a));

    use std::collections::{BinaryHeap, HashSet};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
    struct Happiness {
        point: i64,
        a: usize,
        b: usize,
    }

    impl Ord for Happiness {
        fn cmp(&self, other: &Happiness) -> std::cmp::Ordering {
            self.point.cmp(&other.point)
        }
    }

    let mut q = BinaryHeap::new();
    q.push(Happiness {
        point: A[0] + A[0],
        a: 0,
        b: 0,
    });

    let mut used = HashSet::new();
    used.insert((0, 0));

    let mut ans = 0;

    for _ in 0..M {
        let val = q.pop().unwrap();
        ans += val.point;

        if !used.contains(&(val.a + 1, val.b)) && val.a + 1 < N {
            used.insert((val.a + 1, val.b));
            q.push(Happiness {
                point: A[val.a + 1] + A[val.b],
                a: val.a + 1,
                b: val.b,
            });
        }

        if !used.contains(&(val.a, val.b + 1)) && val.b + 1 < N {
            used.insert((val.a, val.b + 1));
            q.push(Happiness {
                point: A[val.a] + A[val.b + 1],
                a: val.a,
                b: val.b + 1,
            });
        }
    }
    println!("{}", ans);
}

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N],
    }
    let mut A = A;
    A.sort_by(|a, b| b.cmp(a));

    // 幸福度の上昇分が c 以上となるような握手の方法が M 通り以上存在するかどうか
    let solve = |c: i64| {
        let mut cnt = 0;
        for i in 0..N {
            // 左手の握手先 i を決めたときに、右手の握手先 j はどこまで「和を c 以上」となるようにできるか、は二分探索できる
            let j = {
                let mut ok = -1;
                let mut ng = N as i64;
                while ng - ok > 1 {
                    let mid = (ok + ng) / 2;
                    if A[i] + A[mid as usize] >= c {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                if ok < 0 {
                    None
                } else {
                    Some(ok)
                }
            };
            if let Some(j) = j {
                cnt += j + 1;
            }
            if cnt >= M as i64 {
                return true;
            }
        }
        false
    };

    let ok = {
        let mut ok: i64 = 0;
        let mut ng: i64 = 1 << 60;
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if solve(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        dbg!(ok, ng);
        ok
    };

    // 累積和
    let mut sums = Vec::new();
    sums.push(0);
    for i in 0..N {
        sums.push(sums.last().unwrap() + A[i]);
    }

    let mut sum_cnt = 0;
    let mut ans = 0;
    for i in 0..N {
        // 左手の握手先を i と決定したときに、右手の握手先 j を、以下の条件を満たす値として求める
        // 条件: A[i] + A[j] >= ok + 1
        // Aは降順ソート済みであるため、このような j の限界値を二分探索で求めることができる
        // ok ではなく ok + 1 としているのは、ok に等しくなるような組み合わせの個数を数えてしまうと、M 回を超えてしまう可能性があるため。
        // あとで握手回数が M 回になるように ok の値を足したほうが見通しが良い
        let opj = {
            let mut okj = -1;
            let mut ngj = N as i64;
            while ngj - okj > 1 {
                let mid = (okj + ngj) / 2;
                if A[i] + A[mid as usize] >= ok + 1 {
                    okj = mid;
                } else {
                    ngj = mid;
                }
            }
            if okj < 0 {
                None
            } else {
                Some(okj)
            }
        };
        if let Some(j) = opj {
            sum_cnt += j + 1;
            ans += sums[j as usize + 1] + (j + 1) * A[i];
        }
    }

    // M 回に満たない分を足す
    ans += ok * (M as i64 - sum_cnt);

    println!("{}", ans);
}
