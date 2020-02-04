use libprocon::*;

fn main() {
    input! {
        A: usize,
        K: usize,
    }

    let wanted = 2_000_000_000_000;
    if K == 0 {
        println!("{}", wanted - A);
        return;
    }

    let mut d = 0;
    let mut money = A;
    while money < wanted {
        d += 1;
        money += 1 + K * money;
    }
    println!("{}", d);
}
