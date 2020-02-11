use libprocon::*;

// 解説AC
// めぐる式二分探索 cf. [二分探索アルゴリズムを一般化 〜 めぐる式二分探索法のススメ 〜 - Qiita](https://qiita.com/drken/items/97e37dd6143e33a64c8c)
fn main() {
    input! {
        A: i64,
        B: i64,
        X: i64,
    }

    // 買えない場合をチェックする
    if A + B > X {
        println!("0");
        return;
    }

    let max_price: i64 = 1_000_000_000;

    // 10^9 を買えるかどうかチェックする
    if X >= A * 10i64.pow(9u32) + B * 10 {
        println!("{}", max_price);
        return;
    }

    let purchasable = |n: i64| {
        let n_digits = n.to_string().chars().collect::<Vec<_>>().len() as i64;
        X >= A * n + B * n_digits
    };

    // 二分探索する
    let mut ng = max_price;
    let mut ok = 0;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if purchasable(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

// 以下、自力解法だが、1つのWAを潰すためにテストケースをカンニングしてしまった
// 猛省すべし
// fn main() {
//     input! {
//         A: usize,
//         B: usize,
//         X: usize,
//     }

//     // 買えない場合をチェックする
//     if A + B > X {
//         println!("0");
//         return;
//     }

//     // 桁数dを二分探索する
//     let digit = (1..=10).collect::<Vec<usize>>().binary_search_by(|&d| {
//         use std::cmp::Ordering;
//         if 10usize.pow(d as u32) * A + B * (d + 1) <= X {
//             Ordering::Less
//         } else if 10usize.pow(d as u32 - 1) * A + B * d > X {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     });

//     match digit {
//         Err(_) => println!("1000000000"),
//         Ok(d) => {
//             let ans = (X - B * dbg!(d + 1)) / A;
//             // 上の算出方法だと、求めた桁数 d+1 をオーバーしてしまう場合があるため、チェックする
//             let ans = if 10usize.pow(d as u32 + 1) <= ans {
//                 10usize.pow(d as u32 + 1) - 1
//             } else {
//                 ans
//             };
//             println!("{}", min(1000000000, ans));
//         }
//     };
// }
