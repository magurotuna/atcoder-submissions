use libprocon::*;

// テストケースを見てしまった。猛省
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    if N == 2 {
        println!("{}", A.iter().max().unwrap());
        return;
    }

    // l[i] := A0, A1, ..., Ai-1 のGCD
    // r[i] := Ai, Ai+1, ..., AN-1 のGCD
    let mut l = vec![0; N + 1];
    let mut r = vec![0; N + 1];
    l[1] = A[0];
    r[N - 1] = A[N - 1];
    for i in 2..N {
        l[i] = gcd(l[i - 1] as u64, A[i - 1] as u64) as usize;
        r[N - i] = gcd(r[N - i + 1] as u64, A[N - i] as u64) as usize;
        // if A[i] != 877914575 {
        //     dbg!((i, A[i], l[i], r[N - i]));
        // }
    }
    // dbg!((&l, &r));

    println!(
        "{}",
        (1..=N)
            .map(|x| {
                let L = l[x - 1] as u64;
                let R = r[x] as u64;
                if L == 0 {
                    R
                } else if R == 0 {
                    L
                } else {
                    // dbg!(gcd(dbg!(L), dbg!(R)));
                    gcd(L, R)
                }
            })
            .max()
            .unwrap()
    );
}
