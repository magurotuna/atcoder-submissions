use libprocon::*;

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    if n >= 10 {
        println!("{}", r);
        return;
    } else {
        println!("{}", r + 100 * (10 - n));
        return;
    }
}
