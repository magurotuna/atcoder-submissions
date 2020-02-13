use libprocon::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    // a以上b以下の整数は a-b+1 個
    // そのうち、cの倍数であるもの、dの倍数であるもの、LCM(c, d)の倍数であるもの、の個数を求めて、
    // いい感じに集合の演算をすればおｋ
    let cc = count(a, b, c);
    let dd = count(a, b, d);
    let ccdd = count(a, b, lcm(c as u64, d as u64) as usize);
    println!("{}", b - a + 1 - cc - dd + ccdd);
}

// a以上b以下の整数のうち、mの倍数であるものの個数を二分探索でカウントする
fn count(a: usize, b: usize, m: usize) -> usize {
    dbg!((a, b, m));
    if m > b {
        return 0;
    }
    let lower = {
        let mut ng: usize = 0;
        let mut ok: usize = (b + 1000) / m;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            let v = mid * m;
            if a <= v {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };
    let upper = {
        let mut ok: usize = 0;
        let mut ng: usize = (b / m) + 1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let v = mid * m;
            if v <= b {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };
    dbg!(upper) - dbg!(lower) + 1
}
