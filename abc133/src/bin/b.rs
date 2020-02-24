use libprocon::*;

fn main() {
    input! {
        N: usize,
        D: usize,
        X: [[i64; D]; N],
    }

    let mut ans = 0;
    for i in 0..(N - 1) {
        for j in (i + 1)..N {
            let dd = (0..D).map(|k| (X[i][k] - X[j][k]).pow(2u32)).sum::<i64>();
            for x in 1..10000 {
                if x * x == dd {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
