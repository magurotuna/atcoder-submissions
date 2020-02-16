use libprocon::*;
use std::cmp::max;

fn main() {
    let N = read!(usize);
    let mut A: Vec<usize> = Vec::with_capacity(N);
    let mut xy = vec![];
    for _ in 0..N {
        let a = read!(usize);
        A.push(a);
        let tmp = read!(usize, usize; a);
        xy.push(tmp);
    }

    let mut ans = 0;

    // bit全探索
    'outer: for i in 0..(1 << N) {
        let mut honest = vec![];
        let mut n_honest = 0;
        for j in 0..N {
            let is_honest = i & (1 << j) != 0;
            if is_honest {
                n_honest += 1;
            }
            honest.push(is_honest);
        }
        dbg!(n_honest);

        for k in 0..xy.len() {
            for &(x, y) in xy[k].iter() {
                let x = x - 1;
                if honest[k] {
                    if (y == 1) && !honest[x] {
                        continue 'outer; // 矛盾
                    }
                    if (y == 0) && honest[x] {
                        continue 'outer; // 矛盾
                    }
                }
            }
        }
        // 矛盾がなかったらここにくる
        ans = max(ans, n_honest);
    }
    println!("{}", ans);
}
