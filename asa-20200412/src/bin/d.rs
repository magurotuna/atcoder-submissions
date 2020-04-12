use std::cmp::max;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let N = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut tests = Vec::with_capacity(N);
    for _ in 0..N {
        handle.read_line(&mut buf).unwrap();
        let A = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut t = Vec::with_capacity(A);
        for _ in 0..A {
            handle.read_line(&mut buf).unwrap();
            let tup = {
                let tmp = buf
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (tmp[0] - 1, tmp[1] == 1)
            };
            t.push(tup);
            buf.clear();
        }
        tests.push(t);
    }

    let mut ans = 0;
    'outer: for i in 0..(1 << N) {
        let mut n_honests = 0;
        for j in 0..N {
            if i & (1 << j) != 0 {
                // j は正直者
                n_honests += 1;

                for &(x, y) in &tests[j] {
                    let suppose = (i & 1 << x) != 0;
                    if y != suppose {
                        continue 'outer;
                    }
                }
            }
        }
        // 矛盾がなければOK
        ans = max(ans, n_honests);
    }

    println!("{}", ans);
}
