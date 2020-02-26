use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
        S: chars,
    }
    let mut rl = Vec::with_capacity(N);
    let mut cur = S[0];
    let mut len = 0;
    for i in 0..N {
        if S[i] == cur {
            len += 1;
        } else {
            rl.push(len);
            len = 1;
            cur = S[i];
        }
    }
    rl.push(len);

    let mut rl_e = Vec::with_capacity(N);
    if S[0] == '0' {
        rl_e.push(0);
    }
    rl_e.append(&mut rl);
    if S[N - 1] == '0' {
        rl_e.push(0);
    }

    dbg!(&rl_e);

    let mut prev_val: usize = rl_e.iter().take(2 * K + 1).sum();
    let mut ans = prev_val;
    use std::cmp::max;
    let mut idx = 2;
    dbg!(prev_val);
    while idx <= rl_e.len() - 1 {
        let p2 = rl_e[idx - 2];
        let p1 = rl_e[idx - 1];
        let n1 = rl_e.get(idx + 2 * K - 1).unwrap_or(&0);
        let n2 = rl_e.get(idx + 2 * K).unwrap_or(&0);
        dbg!((p2, p1, n1, n2));
        let val = prev_val - p2 - p1 + n1 + n2;
        dbg!((idx, val));
        ans = max(ans, val);
        prev_val = val;
        idx += 2;
    }

    println!("{}", ans);
}
