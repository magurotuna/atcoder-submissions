//! https://github.com/hatoo/competitive-rust-snippets
//!
//! MIT License
//!
//! Copyright (c) 2018 hatoo
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
#![allow(
    unused_imports,
    unused_attributes,
    unused_macros,
    dead_code,
    non_snake_case
)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;
#[macro_export]
macro_rules ! get {(@ inner [$ src : expr ] chars ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . chars () . collect ::< Vec < char >> () } } ; (@ inner [$ src : expr ] usize1 ) => {{get ! (@ inner [$ src ] usize ) - 1 } } ; (@ inner [$ src : expr ] [usize1 ] ) => {{get ! (@ inner [$ src ] [usize ] ) . into_iter () . map (| v | v - 1 ) . collect ::< Vec < usize >> () } } ; (@ inner [$ src : expr ] [[usize1 ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [usize1 ] ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [usize1 ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [usize1 ] ) ) . flatten () . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [[chars ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] chars ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [chars ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] chars ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [($ ($ tt : tt ) ,* ) ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] ($ ($ tt ) ,* ) ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] ($ ($ tt : tt ) ,* ) ) => {{let mut buf : String = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; let mut iter = buf . split_whitespace () ; ($ (get ! (@ inner_elem_parse [$ tt ] iter . next () . unwrap () ) , ) * ) } } ; (@ inner [$ src : expr ] [$ t : ty ] ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . split_whitespace () . map (| t | t . parse ::<$ t > () . unwrap () ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [[$ t : ty ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [$ t ] ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [$ t : ty ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [$ t ] ) ) . flatten () . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] $ t : ty ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . split_whitespace () . next () . unwrap () . parse ::<$ t > () . unwrap () } } ; (@ inner_elem_parse [usize1 ] $ elem : expr ) => {{get ! (@ inner_elem_parse [usize ] $ elem ) - 1 } } ; (@ inner_elem_parse [$ t : ty ] $ elem : expr ) => {{$ elem . parse ::<$ t > () . unwrap () } } ; ($ tt : tt ) => {{use std :: io :: BufRead ; let get_stdin = std :: io :: stdin () ; let mut locked_stdin = get_stdin . lock () ; get ! (@ inner [& mut locked_stdin ] $ tt ) } } ; }
macro_rules ! debug {($ ($ a : expr ) ,* $ (, ) * ) => {# [cfg (debug_assertions ) ] eprintln ! (concat ! ($ ("| " , stringify ! ($ a ) , "={:?} " ) ,*, "|" ) , $ (&$ a ) ,* ) ; } ; }
macro_rules ! echo {($ ($ a : expr ) ,* ) => {let mut s = Vec :: new () ; $ (s . push (format ! ("{}" , $ a ) ) ; ) * println ! ("{}" , s . join (" " ) ) ; } }
#[macro_export]
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
#[macro_export]
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[macro_export]
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
const BIG_STACK_SIZE: bool = true;
fn main() {
    use std::thread;
    if BIG_STACK_SIZE {
        thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .name("solve".into())
            .spawn(solve)
            .unwrap()
            .join()
            .unwrap();
    } else {
        solve();
    }
}

fn solve() {
    let N = get!(usize);
    let As = get!([i64]);
    let mut counts = BTreeMap::new();
    let mut one_cnt = 0;
    for &a in &As {
        if a == 1 {
            one_cnt += 1;
        }
        *counts.entry(a).or_insert(0i64) += 1;
    }
    if one_cnt == 1 {
        echo!(1);
        return;
    }
    if one_cnt >= 2 {
        echo!(0);
        return;
    }

    let mut unique_a = Vec::new();
    let mut multi = HashSet::new();
    for (key, cnt) in counts {
        if cnt >= 2 {
            multi.insert(key);
        }
        unique_a.push(key);
    }

    if unique_a.is_empty() {
        echo!(0);
        return;
    }

    debug!(unique_a);
    let mut rest_a: HashSet<_> = unique_a.clone().into_iter().collect();
    let max_a = *unique_a.last().unwrap();

    for a in unique_a {
        if !rest_a.contains(&a) {
            continue;
        }
        let mut mul = 2;
        while a * mul <= max_a {
            let cur_a = a * mul;
            debug!(cur_a);
            rest_a.remove(&cur_a);
            mul += 1;
        }
    }
    debug!(rest_a);
    let mut ans = 0;
    for ra in rest_a {
        if !multi.contains(&ra) {
            ans += 1;
        }
    }
    echo!(ans);

    //let mut prev_fac = HashMap::new();
    //for a in unique_a {
    //let fac = factorize(a);
    //let mut ok = true;
    //for (&key, _val) in &fac {
    //if prev_fac.contains_key(&key) {
    //ok = false;
    //break;
    //}
    //}
    //if ok {
    //debug!(a);
    //ans += 1;
    //}

    //for (&key, &val) in &fac {
    //*prev_fac.entry(key).or_insert(0) += val;
    //}
    //}
    //echo!(ans);
}

fn solve2() {
    let N = get!(usize);
    let As = get!([i64]);
    let mut factors = HashMap::new();
    let As = As.into_iter().map(|x| factorize(x)).collect::<Vec<_>>();

    for a in &As {
        for (&key, &val) in a {
            *factors.entry(key).or_insert(0) += val;
        }
    }
    debug!(As);
    debug!(factors);

    let mut ans = 0;
    'a: for a in &As {
        for (key, &val) in a {
            if *factors.get(key).unwrap() != val {
                continue 'a;
            }
        }
        ans += 1;
    }
    echo!(ans);
}

pub trait Int:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::hash::Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
    fn next(self) -> Self;
    fn prev(self) -> Self;
    fn sqrt_floor(self) -> Self {
        if self < Self::zero() {
            return Self::zero();
        }
        let two = Self::one().next();
        let mut ok = Self::zero();
        let mut ng = self.next();
        while ng - ok > Self::one() {
            let mid = (ng + ok) / two;
            if mid * mid <= self {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        ((*self) / (*other), (*self) % (*other))
    }
    fn div_floor(&self, other: &Self) -> Self {
        let (d, r) = self.div_rem(other);
        if (r > Self::zero() && *other < Self::zero())
            || (r < Self::zero() && *other > Self::zero())
        {
            d.prev()
        } else {
            d
        }
    }
    fn mod_floor(&self, other: &Self) -> Self {
        let r = *self % *other;
        if (r > Self::zero() && *other < Self::zero())
            || (r < Self::zero() && *other > Self::zero())
        {
            r + *other
        } else {
            r
        }
    }
}
macro_rules ! impl_int_for_numerics {($ ($ t : ty ) * ) => {$ (impl Int for $ t {fn zero () -> Self {0 } fn one () -> Self {1 } fn next (self ) -> Self {self + Self :: one () } fn prev (self ) -> Self {self - Self :: one () } } ) * } }
impl_int_for_numerics ! (u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Int,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Int,
{
    a / gcd(a, b) * b
}
pub fn divosors<T>(n: T) -> Vec<T>
where
    T: Int,
{
    let mut ret = Vec::new();
    let mut cur = T::one();
    loop {
        if cur * cur > n {
            break;
        }
        if n % cur == T::zero() {
            ret.push(cur);
            if cur * cur != n {
                ret.push(n / cur);
            }
        }
        cur = cur.next();
    }
    ret.sort_unstable();
    ret
}
/// Time complexity: O(n log log n)
pub fn lower_primes<T>(n: T) -> Vec<T>
where
    T: Int,
{
    let mut ret = Vec::new();
    if n <= T::one() {
        return ret;
    }
    let mut deque = std::collections::VecDeque::new();
    let mut t = T::one().next();
    while t <= n {
        deque.push_back(t);
        t = t.next();
    }
    let mut p = match deque.pop_front() {
        Some(x) => x,
        None => return ret,
    };
    ret.push(p);
    while p * p <= n {
        deque = deque
            .iter()
            .filter(|&&x| x % p != T::zero())
            .copied()
            .collect();
        p = match deque.pop_front() {
            Some(x) => x,
            None => return ret,
        };
        ret.push(p);
    }
    for n in deque {
        ret.push(n);
    }
    ret
}
/// Time complexity: O(sqrt(n))
pub fn factorize<T>(n: T) -> std::collections::HashMap<T, usize>
where
    T: Int,
{
    let mut ret = std::collections::HashMap::new();
    if n <= T::one() {
        return ret;
    }
    let mut n = n;
    let mut cur = T::one().next();
    loop {
        if cur * cur > n {
            break;
        }
        if n % cur != T::zero() {
            cur = cur.next();
            continue;
        }
        let mut exp = 0;
        while n % cur == T::zero() {
            exp += 1;
            n = n / cur;
        }
        ret.insert(cur, exp);
    }
    if n != T::one() {
        ret.insert(n, 1);
    }
    ret
}
