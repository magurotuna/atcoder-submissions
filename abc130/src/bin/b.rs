use libprocon::*;

fn main() {
    input! {
        N: usize,
        X: usize,
        L: [usize; N],
    }

    let mut ans = 0;
    let mut pos = 0;
    for i in 1..=(N + 1) {
        if pos <= X {
            ans += 1;
        }
        if i != N + 1 {
            pos = pos + L[i - 1];
        }
    }
    println!("{}", ans);
}
