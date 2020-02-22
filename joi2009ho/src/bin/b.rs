use libprocon::*;
use std::cmp::min;

fn main() {
    input! {
        D: usize,
        n: usize,
        m: usize,
        d: [usize; n-1],
        k: [usize; m],
    }
    let d = {
        let mut d_orig = d;
        d_orig.sort();
        let mut dd = Vec::with_capacity(n);
        dd.push(0); // 本店
        dd.append(&mut d_orig);
        dd
    };
    let mut ans = 0;
    for i in 0..m {
        let pos = k[i];
        // pos がどの店舗とどの店舗の間にあるか二分探索
        let index = {
            let mut ok = 0;
            let mut ng = d.len();
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if d[mid] <= pos {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        dbg!((pos, index));

        if index == d.len() - 1 {
            // この場合はindex番目の店舗と本店とのどちらか近い方を加算する
            ans += min(pos - d[index], D - pos);
        } else {
            ans += min(pos - d[index], d[index + 1] - pos);
        }
    }

    println!("{}", ans);
}
