use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
        M: usize,
        A: [usize; N-1],
    }

    println!(
        "{}",
        match (N * M).checked_sub(A.iter().sum()) {
            None => 0 as i64,
            Some(x) if x > K => -1 as i64,
            Some(x) => x as i64,
        }
    );
}
