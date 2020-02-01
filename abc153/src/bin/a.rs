use libprocon::*;

fn main() {
    let (h, a) = read!(f64, f64);
    println!("{}", ((h - 1.0) / a) as usize + 1);
}
