use libprocon::*;

fn solve(n: usize, memo: &mut Vec<i64>) -> i64 {
    if memo[n] != -1 {
        return memo[n];
    }
    let a = solve(n - 1, memo) + solve(n - 2, memo) + solve(n - 3, memo);
    let a = a % 10007;
    memo[n] = a;
    a
}

fn main() {
    input! {
        n: usize,
    }

    let mut tri = vec![-1; 1_000_008];
    tri[1] = 0;
    tri[2] = 0;
    tri[3] = 1;
    println!("{}", solve(n, &mut tri));
}
