use libprocon::*;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let a_len = A.len();

    use std::collections::HashSet;
    let uniq: HashSet<usize> = A.into_iter().collect();

    println!("{}", if a_len == uniq.len() { "YES" } else { "NO" });
}
