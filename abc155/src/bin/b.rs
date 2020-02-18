use libprocon::*;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = true;
    for a in A {
        if a % 2 == 0 {
            if a % 3 == 0 || a % 5 == 0 {
                continue;
            } else {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans { "APPROVED" } else { "DENIED" });
}
