use libprocon::*;

fn count_use_n(n: usize) -> usize {
    // 数字nとn以下の数字mを使用した(n, m)で条件を満たすものの個数を求める関数
    let n_chars = format!("{}", n).chars().collect::<Vec<_>>();
    let len = n_chars.len();
    let first_digit: usize = (n_chars[0] as usize) - 48;
    let last_digit: usize = (n_chars[len - 1] as usize) - 48;

    let mut val = 0;
    if first_digit == last_digit {
        if len == 2 {
            // この場合は n == m のときが条件を満たす
            // また、m == first_digit （1桁）でも条件を満たす。このときはn,mの入れ替えによって2通りある
            // 以上を合計してとりあえず3通りは存在している
            val += 3; // e.g. (11, 11), (11, 1), (1, 11)
            return val;
        } else if len == 3 {
            // 2桁、1桁のmに対しては入れ替えも含め4通りある
            val += 4;
            // mが3桁のときはnの真ん中の数字によって個数が決まる
            let tmp = (n_chars[1] as usize) - 48; // e.g. 181 だとすると 8

            // 入れ替えと n == m を考慮
            val += 2 * tmp + 1;
        } else {
            // 2桁、1桁のmに対しては入れ替えも含め4通りある
            val += 4;

            // 3桁〜(len - 1)桁
            for i in 4..=len {
                val += 10usize.pow((i - 3) as u32) * 2; // 入れ替え考慮
            }

            // nとmの桁数が一致するときの数をカウント
            let tmp: usize = n_chars[1..(len - 1)]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap(); // e.g. 1231 だとすると 23
            val += tmp * 2 + 1; // 入れ替えと自分自身
        }
    } else if first_digit < last_digit {
        // e.g. n = 123 のとき、m としては 31 しか適するものはありえない。
        // 入れ替えも考え、2通り
        // ただし、nが2桁のときは無理
        if len >= 3 {
            val += 2;
        }

        // e.g. n = 1234 のとき、mが3桁とすると 4[]1 が作れる。
        if len >= 4 {
            for i in 4..=len {
                val += 10usize.pow((i - 3) as u32) * 2; // 入れ替えを考慮して2倍する
            }
        }
    } else {
        if last_digit != 0 {
            // e.g. n = 321 のとき、3桁のmを考えると、 m = 1 [] 3
            // となるが、このとき[]に何をいれてもn以下の数字になるから問題ない
            // ただし、nが2桁のときは[]の枠が存在しないので何もしない
            if len >= 3 {
                for i in 3..=len {
                    val += 10usize.pow((i - 2) as u32) * 2; // 入れ替えを考慮して2倍する
                }
            }

            // (n, m) = (321, 13), (13, 321) のように2桁のパターン
            val += 2;
        }
    }

    val
}

fn main() {
    input! {
        N: usize,
    }

    if N <= 9 {
        println!("{}", N);
        return;
    }

    let mut dp = vec![0; N + 1];
    for i in 1..=9 {
        dp[i] = i;
    }

    for i in 10..=N {
        let val = count_use_n(i);
        dp[i] = dp[i - 1] + val;
    }

    println!("{}", dp[N]);
}
