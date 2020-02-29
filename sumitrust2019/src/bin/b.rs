use libprocon::*;

fn main() {
    input! {
        n: i64,
    }
    let mut ans = -1;
    for x in 1..=n {
        let v = x * 108 / 100;
        dbg!((x, v));
        if v == n {
            ans = x;
        }
    }
    if ans == -1 {
        println!(":(");
    } else {
        println!("{}", ans);
    }
}
