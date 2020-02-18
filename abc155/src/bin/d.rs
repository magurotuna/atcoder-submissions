use libprocon::*;

fn main() {
    input! {
        N: i64,
        K: i64,
        mut a: [i64; N],
    }
    a.sort();
}

// コンテスト中の解答 提出までたどり着けなかった
fn my_solution() {
    input! {
        N: i64,
        K: i64,
        A: [i64; N],
    }
    let mut A = A;
    A.sort();

    // 負領域の上限のインデックスと正領域の下限のインデックスを二分探索する
    let upper_neg = {
        let mut ok = -1;
        let mut ng = N as i64;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if A[mid as usize] < 0 {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok + 1
    };
    let lower_pos = {
        let mut ok = N as i64;
        let mut ng = -1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if A[mid as usize] > 0 {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };
    dbg!((upper_neg, lower_pos));

    // ペアの積が負になるもの、0になるもの、正になるもの、が何個あるか計算
    let p_neg = (N - lower_pos) * upper_neg;
    let p_zero = (lower_pos - upper_neg) * (N - (lower_pos - upper_neg))
        + (lower_pos - upper_neg) * (lower_pos - upper_neg - 1) / 2;
    let p_pos = ((N - lower_pos) * (N - lower_pos - 1) / 2) + ((upper_neg - 1) * upper_neg / 2);
    dbg!((p_neg, p_zero, p_pos));

    if K <= p_neg {
        // 積が負になる組み合わせのうち、K番目に小さいもの
        let mut left = 0;
        let mut right = (N - 1) as usize;
        let mut count = 0;
        loop {
            count += 1;
            dbg!((left, right));
            let prod = A[left] * A[right];
            if count == K {
                println!("{}", prod);
                return;
            }

            if A[left] * A[right - 1] >= A[left + 1] * A[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
    } else if p_neg + 1 <= K && K <= p_neg + p_zero {
        println!("0");
        return;
    } else {
        // 積が正になる組み合わせのうち、K - p_neg - p_zero 番目に小さいもの
        let mut dp = vec![vec![]];
        dp[0].push(1);
    }
}
