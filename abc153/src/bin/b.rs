use libprocon::*;

fn main() {
    let (h, n) = read!(usize, usize);
    let a: Vec<usize> = read![[usize]];

    let s: usize = a.into_iter().sum();
    println!("{}", if h <= s { "Yes" } else { "No" });
}
