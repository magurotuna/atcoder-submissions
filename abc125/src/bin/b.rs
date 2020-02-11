use libprocon::*;

fn main() {
    input! {
        N: usize,
        v: [usize; N],
        c: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if v[i] > c[i] {
            ans += v[i] - c[i];
        }
    }
    println!("{}", ans);
}
