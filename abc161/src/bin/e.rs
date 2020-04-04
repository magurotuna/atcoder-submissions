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

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: chars,
    }

    if n == 1 {
        if s[0] == 'o' {
            println!("1");
        }
        return;
    }

    if n == 2 {
        if k == 1 {
            if s[0] == 'o' && s[1] == 'o' {
                return;
            }
            if s[0] == 'o' {
                println!("1");
                return;
            }
            if s[1] == 'o' {
                println!("2");
                return;
            }
            return;
        }
        if s[0] == 'o' && s[1] == 'o' && c == 0 {
            println!("1\n2");
            return;
        }
        return;
    }

    // i 日目以降に条件を満たすようにして何回働くことが可能かを保持する
    let mut dp1 = vec![0; n + 1];
    let mut tmp = 0;
    for i in (1..=n).rev() {
        if s[i - 1] == 'o' {
            dp1[i] += 1;
            if i + c + 1 < n + 1 {
                dp1[i] += dp1[i + c + 1];
            }
            tmp = dp1[i];
        } else {
            dp1[i] = tmp;
        }
    }

    // i 日目以前に条件を満たすようにして何回働くことが可能かを保持する
    let mut dp2 = vec![0; n + 1];
    tmp = 0;
    for i in (1..=n) {
        if s[i - 1] == 'o' {
            dp2[i] += 1;
            if i > c + 1 {
                dp2[i] += dp2[i - c - 1];
            }
            tmp = dp2[i];
        } else {
            dp2[i] = tmp;
        }
    }

    let mut ans = Vec::new();
    for i in 1..=n {
        // i 日目に働かない場合に、労働日数を満たすように働くことができるかを見ればよい
        if 2 <= i && i <= n - 1 {
            if dp2[i - 1] + dp1[i + 1] < k {
                ans.push(i);
            }
            continue;
        }
        if i == 1 {
            if dp1[i + 1] < k {
                ans.push(i);
            }
            continue;
        }
        if i == n {
            if dp2[i - 1] < k {
                ans.push(i);
            }
            continue;
        }
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
