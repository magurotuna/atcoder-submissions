use libprocon::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize,
    }
    let mut ans = 0;
    for i in 1..=t {
        if i % a == 0 {
            ans += b;
        }
    }
    println!("{}", ans);
}
