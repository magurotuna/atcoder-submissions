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
        N: usize,
        xy: [(i64, i64); N],
    }
    let mut data = Vec::with_capacity(N);
    for i in 0..N {
        data.push(i);
    }
    let case = calc(N);
    let mut perm = Vec::with_capacity(case);
    loop {
        perm.push(data.clone());
        if !data.next_permutation() {
            break;
        }
    }
    let mut ans = 0.0;
    for arr in perm {
        let mut tmp = 0.0;
        for i in 0..(arr.len() - 1) {
            let from = arr[i];
            let to = arr[i+1];
            let (fx, fy) = xy[from];
            let (tx, ty) = xy[to];
            tmp += (((fx - tx).pow(2u32) + (fy - ty).pow(2u32)) as f64).sqrt();
        }
        ans += (tmp as f64) / (case as f64);
    }
    println!("{}", ans);
}

fn calc(a: usize) -> usize {
    if a == 0 {
        1
    } else {
        a * calc(a - 1)
    }
}

/// Permute a slice into its next or previous permutation (in lexical order).
///
/// ```
/// use permutohedron::LexicalPermutation;
///
/// let mut data = [1, 2, 3];
/// let mut permutations = Vec::new();
///
/// loop {
///     permutations.push(data.to_vec());
///     if !data.next_permutation() {
///         break;
///     }
/// }
///
/// assert_eq!(permutations, &[&[1, 2, 3], &[1, 3, 2],
///                            &[2, 1, 3], &[2, 3, 1],
///                            &[3, 1, 2], &[3, 2, 1]]);
///
/// // `data` has been mutated in-place:
/// assert_eq!(data, [3, 2, 1]);
/// ```
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }

}

#[test]
fn lexical() {
    let mut data = [1, 2, 3];
    data.next_permutation();
    assert_eq!(&data, &[1, 3, 2]);
    data.next_permutation();
    assert_eq!(&data, &[2, 1, 3]);
    data.prev_permutation();
    assert_eq!(&data, &[1, 3, 2]);
    data.prev_permutation();
    assert_eq!(&data, &[1, 2, 3]);
    assert!(!data.prev_permutation());
    let mut c = 0;
    while data.next_permutation() {
        c += 1;
    }
    assert_eq!(c, 5);
}
