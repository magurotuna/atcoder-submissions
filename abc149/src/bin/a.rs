use libprocon::*;

fn main() {
    input! {
        S: chars,
        T: chars,
    }
    let mut T = T;
    let mut S = S;
    T.append(&mut S);
    println!("{}", T.into_iter().collect::<String>());
}
