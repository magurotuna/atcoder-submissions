use libprocon::*;

fn main() {
    input! {
        x: usize,
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 1..=x {
        dp[i] = check(&dp, i);
    }
    println!("{}", if dp[x] { 1 } else { 0 });
}

fn check(table: &[bool], i: usize) -> bool {
    (100..=105).map(|n| sub_check(table, i, n)).any(|b| b)
}

fn sub_check(table: &[bool], i: usize, sub: usize) -> bool {
    match i.checked_sub(sub) {
        Some(x) => *table.get(x).unwrap_or(&false),
        None => false,
    }
}

fn my_solution() {
    input! {
        x: i64,
    }

    if x >= 2000 {
        println!("1");
        return;
    }

    let r = x % 100;
    let h = (x - r) / 100;

    if r <= 5 * h {
        println!("1");
        return;
    } else {
        println!("0");
        return;
    }
}
