use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; N],
    }
    // まず累積和を求めておく
    let mut cum_sum = Vec::with_capacity(N + 1);
    cum_sum.push(0usize);
    for i in 0..N {
        cum_sum.push(a[i] + cum_sum[i]);
    }

    let mut ans = 0;
    // スタート位置をiで固定して、和がK以上となる最小のインデックスjを二分探索する
    // jが見つかったら、このiに対してはN-j個の部分列が条件を満たすということになる
    for i in 0..N {
        let mut ok = N as i64;
        let mut ng = -1i64;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(&cum_sum, i, mid, K) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += N - ok as usize;
    }
    println!("{}", ans);
}

fn is_ok(cum_sum: &[usize], i: usize, j: i64, K: usize) -> bool {
    if j < 0 {
        false
    } else {
        let a: usize = j as usize + 1;
        cum_sum[a].saturating_sub(cum_sum[i]) >= K
    }
}
