use libprocon::*;

fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    // 1があればN-1個砕いて1を残せば満足させられる
    // 1がないと絶対無理
    // 1,2,3...というように1ずつ増加する部分列の長さを求める
    let mut cnt = 0;
    for i in 0..N {
        if a[i] == cnt + 1 {
            cnt += 1;
        }
    }
    dbg!(cnt);
    println!("{}", if cnt == 0 { -1 } else { N as i64 - cnt as i64 });
}
