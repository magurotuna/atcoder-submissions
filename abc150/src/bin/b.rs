use libprocon::*;

fn main() {
    input! {
        N: usize,
        S: chars,
    }
    let mut ans = 0;
    for i in 0..(N - 2) {
        let s = S.get(i..(i + 3)).unwrap().iter().collect::<String>();
        dbg!(&s);
        if s == "ABC" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
