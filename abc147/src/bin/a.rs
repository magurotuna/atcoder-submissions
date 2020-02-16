use libprocon::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    println!("{}", if a + b + c >= 22 { "bust" } else { "win" });
}
