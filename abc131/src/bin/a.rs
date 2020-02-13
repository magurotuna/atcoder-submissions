use libprocon::*;

fn main() {
    input! {
        s: String,
    }
    let c = s.chars().collect::<Vec<_>>();
    let mut is_bad = false;
    for i in 0..(c.len() - 1) {
        if c[i] == c[i + 1] {
            is_bad = true;
            break;
        }
    }
    println!("{}", if !is_bad { "Good" } else { "Bad" });
}
