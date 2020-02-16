use libprocon::*;

fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let mut p2 = p.clone();
    p2.sort();

    let mut cnt = 0;
    for i in 0..N {
        if p2[i] != p[i] {
            cnt += 1;
        }
    }
    println!("{}", if cnt <= 2 { "YES" } else { "NO" });
}
