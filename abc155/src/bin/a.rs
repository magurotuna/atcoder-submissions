use libprocon::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let pity = (a == b && a != c) || (a == c && a != b) || (b == c && b != a);
    println!("{}", if pity { "Yes" } else { "No" });
}
