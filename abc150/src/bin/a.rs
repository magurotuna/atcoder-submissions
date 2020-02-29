use libprocon::*;

fn main() {
    input! {
        k: i64,
        x: i64,
    }
    println!("{}", if 500 * k >= x { "Yes" } else { "No" });
}
