use libprocon::*;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let s = a + b;
    if s % 2 == 0 {
        println!("{}", s / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
