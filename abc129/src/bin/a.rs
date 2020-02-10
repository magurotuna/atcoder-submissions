use libprocon::*;

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize,
    }
    use std::cmp::min;
    println!("{}", min(p + q, min(q + r, r + p)));
}
