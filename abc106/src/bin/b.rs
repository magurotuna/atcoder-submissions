use libprocon::*;

fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        if i % 2 == 0 {
            continue;
        }
        let t = i.factorize();
        if t.values().map(|v| v + 1).product::<usize>() == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
