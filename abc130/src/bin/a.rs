use libprocon::*;

fn main() {
    input! {
        x: usize,
        a: usize,
    }
    println!("{}", if x < a { 0 } else { 10 });
}
