use libprocon::*;

fn main() {
    input! {
        S: String,
        T: String,
        A: usize,
        B: usize,
        U: String,
    }

    let s_count = if U == S { A - 1 } else { A };
    let t_count = if U == T { B - 1 } else { B };
    println!("{} {}", s_count, t_count);
}
