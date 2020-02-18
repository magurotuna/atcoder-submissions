use libprocon::*;

fn main() {
    input! {
        A: u64,
        B: u64,
    }
    println!("{}", lcm(A, B));
}
