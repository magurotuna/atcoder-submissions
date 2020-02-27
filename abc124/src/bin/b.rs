use libprocon::*;

fn main() {
    input! {
        N: usize,
        H: [i32; N],
    }
    let ans = 1
        + (1..N)
            .map(|n| {
                let h = H[n];
                if H.iter().take(n).all(|&t| dbg!(t) <= dbg!(h)) {
                    dbg!((n, "a"));
                    1
                } else {
                    dbg!((n, "b"));
                    0
                }
            })
            .sum::<i32>();
    println!("{}", ans);
}
