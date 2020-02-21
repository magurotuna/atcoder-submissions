use libprocon::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
        y: usize,
    }
    use std::cmp::{max, min};

    let a_and_b = a * x + b * y;
    let ab_and_rest = if x >= y {
        c * 2 * y + (x - y) * a
    } else {
        c * 2 * x + (y - x) * b
    };
    let all_ab = c * 2 * max(x, y);

    println!("{}", [a_and_b, ab_and_rest, all_ab].iter().min().unwrap());
}
