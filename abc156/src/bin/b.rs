use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut ans = 0;
    let mut n = N;
    while n >= K {
        ans += 1;
        n /= K;
    }
    println!("{}", ans + 1);
}
