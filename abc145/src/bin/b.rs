use libprocon::*;

fn main() {
    input! {
        N: usize,
        S: chars,
    }
    if S.len() % 2 != 0 {
        println!("No");
        return;
    }

    for i in 0..(N / 2) {
        dbg!((i, i + N / 2));
        if S[i] != S[i + N / 2] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
