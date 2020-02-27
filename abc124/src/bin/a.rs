use libprocon::*;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let mut ans = 0;
    let mut a = a;
    let mut b = b;
    ans += if a > b {
        a -= 1;
        a + 1
    } else {
        b -= 1;
        b + 1
    };
    ans += if a > b {
        a -= 1;
        a + 1
    } else {
        b -= 1;
        b + 1
    };
    println!("{}", ans);
}
