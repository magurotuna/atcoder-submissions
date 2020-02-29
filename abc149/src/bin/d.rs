use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
        R: i64,
        S: i64,
        P: i64,
        T: chars,
    }

    let mut ans = 0;
    for k in 0..K {
        let mut seq = Vec::with_capacity(K);
        let mut flag = false;
        for v in T.iter().skip(k).step_by(K) {
            match seq.last() {
                Some(&x) => {
                    if x != v {
                        flag = false;
                        seq.push(v);
                    } else {
                        if flag {
                            flag = false;
                            seq.push(v);
                        } else {
                            flag = true;
                        }
                    }
                }
                None => seq.push(v),
            };
        }
        ans += seq
            .into_iter()
            .map(|s| match s {
                'r' => P,
                's' => R,
                'p' => S,
                _ => 0,
            })
            .sum::<i64>();
    }
    println!("{}", ans);
}
