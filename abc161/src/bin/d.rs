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
        k: usize,
    }
    let mut lunlun = Vec::with_capacity(100_100);
    for i in 1..10 {
        dfs(&mut lunlun, &vec![i]);
    }
    lunlun.sort();
    println!("{}", lunlun[k - 1]);
}

fn dfs(lunlun: &mut Vec<usize>, cur: &Vec<u8>) {
    if vec2num(&cur) > 3234566667 {
        return;
    }

    lunlun.push(vec2num(cur));

    let &last = cur.last().unwrap();

    if last == 0 {
        let mut clone1 = cur.clone();
        clone1.push(0);
        dfs(lunlun, &clone1);
        let mut clone2 = cur.clone();
        clone2.push(1);
        dfs(lunlun, &clone2);
    } else if last == 9 {
        let mut clone1 = cur.clone();
        clone1.push(9);
        dfs(lunlun, &clone1);
        let mut clone2 = cur.clone();
        clone2.push(8);
        dfs(lunlun, &clone2);
    } else {
        let mut clone1 = cur.clone();
        clone1.push(last - 1);
        dfs(lunlun, &clone1);
        let mut clone2 = cur.clone();
        clone2.push(last);
        dfs(lunlun, &clone2);
        let mut clone3 = cur.clone();
        clone3.push(last + 1);
        dfs(lunlun, &clone3);
    }
}

fn vec2num(num: &[u8]) -> usize {
    let mut tmp = 0;
    for i in 0..num.len() {
        tmp *= 10;
        tmp += num[i] as usize;
    }
    tmp
}
