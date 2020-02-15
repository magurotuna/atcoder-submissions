use libprocon::*;

fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut ans = 1;
    let mut position = d;
    while position + d < n - 1 {
        position += 2 * d + 1;
        ans += 1;
    }
    println!("{}", ans);
}
