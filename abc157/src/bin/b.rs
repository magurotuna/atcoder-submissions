#![allow(unused_macros)]

// cf. [Rustで競技プログラミングの入力をスッキリ記述するマクロ - Qiita](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn check(nums: &[i64], row: &mut Vec<i64>) {
    nums.iter().for_each(|&n| {
        for i in 0..row.len() {
            if row[i] == n {
                row[i] = -1;
            }
        }
    })
}

fn main() {
    input! {
        A1: [i64; 3],
        A2: [i64; 3],
        A3: [i64; 3],
        N: usize,
        B: [i64; N],
    }
    let mut A1 = A1;
    let mut A2 = A2;
    let mut A3 = A3;
    check(&B, &mut A1);
    check(&B, &mut A2);
    check(&B, &mut A3);

    dbg!(&A1);
    dbg!(&A2);
    dbg!(&A3);

    // 横に揃っている
    if A1.iter().all(|a| a == &-1) || A2.iter().all(|a| a == &-1) || A3.iter().all(|a| a == &-1) {
        println!("Yes");
        return;
    }

    for i in 0..3 {
        // 縦にそろっている
        if A1[i] == -1 && A2[i] == -1 && A3[i] == -1 {
            println!("Yes");
            return;
        }
    }

    // 斜めに揃っている
    let one = A1[0] == -1 && A2[1] == -1 && A3[2] == -1;
    let two = A1[2] == -1 && A2[1] == -1 && A3[0] == -1;
    if one || two {
        println!("Yes");
        return;
    }

    println!("No");
}
