use libprocon::*;

fn main() {
    input! {
        S: chars,
    }
    let l = S.len();
    let mut ans = 0;
    for i in 0..(l / 2) {
        if S[i] != S[l - 1 - i] {
            dbg!((S[i], S[l - 1 - i]));
            ans += 1;
        }
    }
    println!("{}", ans);
}
