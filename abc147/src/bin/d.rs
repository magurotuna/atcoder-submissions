use libprocon::*;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let MOD = 1e9 as usize + 7;
    let mut bits = vec![0; 60];

    for a in A {
        for j in 0..60 {
            if a & (1 << j) != 0 {
                bits[j] += 1;
            }
        }
    }

    dbg!(&bits);

    let mut ans = 0;
    for j in 0..60 {
        // 2進表記で下からj桁目が1となるようなXORの組み合わせの個数
        let c = bits[j] * (N - bits[j]);
        let c = c % MOD;
        ans += (2usize.pow(j as u32) % MOD) * c;
        ans = ans % MOD;
    }
    println!("{}", ans);
}

// XORは繰り上がりのない演算なので桁ごとに考えることができる
// また、2進数同士の和は、ビットごとに分割して考えることができる（当然繰り上がりはあるが、10進数と同じで、13 + 38 は `10 + 30` と `3 + 8` に分割して和を求めて、最後に和同士の和をとると考えればよい
