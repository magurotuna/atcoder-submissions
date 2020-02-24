use libprocon::*;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    println!("{}", std::cmp::min(a * n, b));
}
