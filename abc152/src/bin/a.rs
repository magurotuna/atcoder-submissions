use libprocon::*;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    println!("{}", if n == m { "Yes" } else { "No" });
}
