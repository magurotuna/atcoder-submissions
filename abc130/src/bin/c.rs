use libprocon::*;

fn main() {
    input! {
        W: usize,
        H: usize,
        x: usize,
        y: usize,
    }
    println!(
        "{} {}",
        (W * H) as f64 / 2.0,
        if 2 * x == W && 2 * y == H { 1 } else { 0 }
    );
}
