use libprocon::*;

// 解説AC
fn main() {
    input! {
        N: String,
        K: usize,
    }
    let d = N
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let d_len = d.len();

    // dp[i][j][k] := d[i]（i桁目）を調べていて、それまでに使った0以外の数字の個数がj個で、それまでに確定している数字がN未満であることが確定しているか否か（確定してたら1）、という値を示すDPテーブル
    let mut dp = vec![vec![vec![0; 2]; 4]; 110];
    dp[0][0][0] = 1;

    for i in 0..d_len {
        for j in 0..4 {
            for k in 0..2 {
                let digit = d[i];
                for x in 0..10 {
                    let next_i = i + 1;
                    let mut next_j = j;
                    let mut next_k = k;
                    if x != 0 {
                        next_j += 1;
                    }
                    if next_j > K {
                        continue;
                    }
                    if k == 0 {
                        if x < digit {
                            next_k = 1;
                        } else if x > digit {
                            continue;
                        }
                    }
                    dp[next_i][next_j][next_k] += dp[i][j][k];
                }
            }
        }
    }
    println!("{}", dp[d_len][K][0] + dp[d_len][K][1]);

    // for i in 1..=d_len {
    //     for small in 0..2 {
    //         for j in 0..=K {
    //             for x in 0..=(if small == 1 { 9 } else { d[i - 1] }) {
    //                 dbg!((i, small, j, x));
    //                 if x == 0 {
    //                     if 0 == d[i - 1] {
    //                         dp[i][0][j] += dp[i - 1][0][j];
    //                     } else if 0 < d[i - 1] {
    //                         dp[i][1][j] += dp[i - 1][0][j];
    //                     }
    //                     dp[i][1][j] += dp[i - 1][1][j];
    //                 } else if j > 0 {
    //                     if x == d[i - 1] {
    //                         dp[i][0][j] += dp[i - 1][0][j - 1];
    //                     } else if x < d[i - 1] {
    //                         dp[i][1][j] += dp[i - 1][0][j - 1];
    //                     }
    //                     dp[i][1][j] += dp[i - 1][1][j - 1];
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("{}", dp[d_len][0][K] + dp[d_len][1][K]);
}

// fn main() {
//     input! {
//         N: String,
//         K: usize,
//     }
//     let d = N.chars().collect::<Vec<char>>();
//     let d_len = d.len();

//     let mut ans = 0usize;

//     for i in 1..=d_len {
//         // i は調べる数字の桁数
//         if i == d_len {
//             // Nと同じ桁数の場合、10^(i-1) 以上 N 以下 の数字の中から、0でない数字がK個であるものの個数を調べる
//             if K == 1 {
//                 ans += d[0].to_digit(10).unwrap() as usize;
//                 continue;
//             } else if K == 2 {
//                 // 上1桁として使える数字の種類数
//                 let first_kinds = d[0].to_digit(10).unwrap() as usize;
//             } else {
//                 // 上1桁として使える数字の種類数
//                 let first_kinds = d[0].to_digit(10).unwrap() as usize;
//             }
//         }

//         if i == 1 {
//             // N が2桁以上で1桁について調べるとき
//             // Kが1のときは1から9の9個が条件に合致
//             if K == 1 {
//                 ans += 9;
//             }
//             continue;
//         }

//         if i == 2 {
//             // N が3桁以上で2桁について調べるとき
//             // Kが3の場合は無理なのでスルー
//             if K == 3 {
//                 continue;
//             } else if K == 2 {
//                 // 10, 20, 30, ..., 90 以外の数ならOK
//                 // 10から99の90個の数字のうち9個がダメだから81個がOK
//                 ans += 81;
//                 continue;
//             } else {
//                 // 10, 20, 30, ..., 90 の9個がOK
//                 ans += 9;
//                 continue;
//             }
//         }

//         // 10^(i-1) 以上 10^i 未満の数字の中から、0でない数字がK個あるものの個数を数える
//         if K == 1 {
//             // 10000, 20000, 30000, ..., 90000 とかが適合する
//             ans += 9;
//             continue;
//         } else if K == 2 {
//             // 0でない数字は必ず上1桁には使用しなければならない -> 9通り
//             // もう1つの数字を入れる箇所がi-1通りあるので、 i * 9 通り
//             ans += 9 * (i - 1) * 9;
//             continue;
//         } else {
//             // 0でない数字は必ず上1桁には使用しなければならない -> 9通り
//             // もう2つの数字を入れる箇所がi-1通りあるので、 (i-1)_C_(2) * 9 * 9 通り
//             ans += 9 * (((i - 1) * (i - 2)) / 2) * 9 * 9;
//             continue;
//         }
//     }

//     println!("{}", ans);
// }
