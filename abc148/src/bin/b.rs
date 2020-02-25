use libprocon::*;

fn main() {
    input! {
        N: usize,
        S: chars,
        T: chars,
    }
    let mut ans = String::new();
    for i in 0..N {
        ans.push(S[i]);
        ans.push(T[i]);
    }
    println!("{}", ans);
}
