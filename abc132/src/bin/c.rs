use libprocon::*;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut d = d;
    d.sort();
    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
