use libprocon::*;

fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut prev = P[0];
    let mut ans = 1;
    if N == 1 {
        println!("1");
        return;
    }
    for i in 1..N {
        if prev >= P[i] {
            dbg!((prev, P[i], ans));
            ans += 1;
            prev = P[i];
        }
    }
    println!("{}", ans);
}
