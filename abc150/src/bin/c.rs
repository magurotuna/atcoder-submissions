use libprocon::*;

fn main() {
    input! {
        N: i32,
        P: [i32; N],
        Q: [i32; N],
    }
    let mut v = (1..=N).collect::<Vec<i32>>();
    let mut a = if equal(&v, &P) { 1 } else { 0 };
    let mut b = if equal(&v, &Q) { 1 } else { 0 };

    let mut cur = 2;
    while v.next_permutation() {
        a = if equal(&v, &P) { cur } else { a };
        b = if equal(&v, &Q) { cur } else { b };
        cur += 1;
    }
    println!("{}", ((a - b) as i32).abs());
}

fn equal(v: &[i32], p: &[i32]) -> bool {
    v.iter().zip(p).all(|(a, b)| a == b)
}

#[doc = " Ported from:"]
#[doc = " - [bluss/permutohedron](https://github.com/bluss/permutohedron)"]
#[doc = " - [hatoo/competitive-rust-snippets](https://github.com/hatoo/competitive-rust-snippets)"]
pub trait LexicalPermutation {
    #[doc = " Return `true` if the slice was permuted, `false` if it is already"]
    #[doc = " at the last ordered permutation."]
    fn next_permutation(&mut self) -> bool;
    #[doc = " Return `true` if the slice was permuted, `false` if it is already"]
    #[doc = " at the first ordered permutation."]
    fn prev_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    #[doc = " Original author in Rust: Thomas Backman <serenity@exscape.org>"]
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}
