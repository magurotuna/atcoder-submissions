use libprocon::*;

fn main() {
    input! {
        N: usize,
        M: i64,
        a: [i64; N],
    }

    let two_included = count_two(a[0]);
    let mut l = 1;
    for i in 0..N {
        if two_included != count_two(a[i]) {
            println!("0");
            return;
        }
        l = lcm(l, a[i]);
    }
    let min_l = l / 2;
    println!("{}", ((M / min_l) + 1) / 2);
}

fn count_two(n: i64) -> i64 {
    let mut n = n;
    let mut cnt = 0;
    while n % 2 == 0 {
        cnt += 1;
        n /= 2;
    }
    cnt
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
