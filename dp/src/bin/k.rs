use libprocon::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    // dp[i] := i個の石からなる山のとき、それが必敗パターンであるか否か
    let mut dp = vec![None; k + 1];
    dp[0] = Some(true);

    for i in 1..=k {
        if a.iter()
            .filter_map(|&x| i.checked_sub(x))
            .all(|x| match dp[x] {
                Some(b) => !b,
                None => false,
            })
        {
            dp[i] = Some(true);
        } else {
            dp[i] = Some(false);
        }
    }
    println!("{}", if dp[k].unwrap() { "Second" } else { "First" });
}
