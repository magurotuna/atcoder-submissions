use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: f64,
    }
    println!("{}", 2.0 * r * std::f64::consts::PI);
}
