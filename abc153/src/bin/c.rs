use libprocon::*;

fn main() {
    let (n, k) = read!(usize, usize);
    let mut h: Vec<usize> = read![[usize]];

    h.sort();
    h.reverse();
    println!("{}", h.into_iter().skip(k).sum::<usize>());
}
