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
    let (n, m) = get!((usize, usize));
    let fac = factorize(m);
    debug!(fac);
    let comb = Comb::new(300_000, 1_000_000_007);
    let mut ans = 1usize.mint();
    for (_, v) in fac {
        ans *= comb.calc(v + n - 1, n - 1);
    }
    echo!(ans);
}

/// 累乗のmod
/// cf. https://github.com/hatoo/competitive-rust-snippets/blob/master/src/modulo.rs
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}
/// mod m での a の逆元を求める
/// m と a が互いに素でなければならないことに注意
/// cf. [「1000000007 で割ったあまり」の求め方を総特集！ 〜 逆元から離散対数まで 〜 - Qiita](https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a)
pub fn mod_inv(val: i64, modulo: i64) -> i64 {
    use std::mem::swap;
    let mut a = val;
    let mut b = modulo;
    let mut ret = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        ret -= t * v;
        swap(&mut ret, &mut v);
    }
    ret %= modulo;
    if ret < 0 {
        ret += modulo;
    }
    ret
}
pub trait IntoModInt {
    /// Create a ModInt instance with modulo 1_000_000_007 (= 10^9 + 7).
    fn mint(&self) -> ModInt;
    /// Create a ModInt instance with any modulo.
    fn mint_with_mod(&self, modulo: Self) -> ModInt;
}
#[derive(Clone, Copy)]
pub struct ModInt {
    value: i64,
    modulo: i64,
}
impl ModInt {
    pub fn new(value: i64, modulo: i64) -> Self {
        let r = value % modulo;
        Self {
            value: if r < 0 { r + modulo } else { r },
            modulo,
        }
    }
}
impl std::fmt::Debug for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl std::ops::Add for ModInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value, self.modulo)
    }
}
impl std::ops::Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value, self.modulo)
    }
}
impl std::ops::Mul for ModInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value, self.modulo)
    }
}
impl std::ops::Div for ModInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let inv = mod_inv(other.value, self.modulo);
        Self::new(self.value * inv, self.modulo)
    }
}
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.value + other.value, self.modulo);
    }
}
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.value - other.value, self.modulo);
    }
}
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::new(self.value * other.value, self.modulo);
    }
}
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        let inv = mod_inv(other.value, self.modulo);
        *self = Self::new(self.value * inv, self.modulo);
    }
}
macro_rules ! impl_mod_int {($ ($ t : ty ) * ) => ($ (impl IntoModInt for $ t {fn mint (& self ) -> ModInt {ModInt :: new (* self as i64 , 1_000_000_007 ) } fn mint_with_mod (& self , modulo : $ t ) -> ModInt {ModInt :: new (* self as i64 , modulo as i64 ) } } impl std :: cmp :: PartialEq <$ t > for ModInt {fn eq (& self , other : &$ t ) -> bool {self . value == (* other as i64 ) } } impl std :: ops :: Add <$ t > for ModInt {type Output = Self ; fn add (self , other : $ t ) -> Self {Self :: new (self . value + (other as i64 ) , self . modulo ) } } impl std :: ops :: AddAssign <$ t > for ModInt {fn add_assign (& mut self , other : $ t ) {* self = Self :: new (self . value + (other as i64 ) , self . modulo ) ; } } impl std :: ops :: Sub <$ t > for ModInt {type Output = Self ; fn sub (self , other : $ t ) -> Self {Self :: new (self . value - (other as i64 ) , self . modulo ) } } impl std :: ops :: SubAssign <$ t > for ModInt {fn sub_assign (& mut self , other : $ t ) {* self = Self :: new (self . value - (other as i64 ) , self . modulo ) ; } } impl std :: ops :: Mul <$ t > for ModInt {type Output = Self ; fn mul (self , other : $ t ) -> Self {Self :: new (self . value * (other as i64 ) , self . modulo ) } } impl std :: ops :: MulAssign <$ t > for ModInt {fn mul_assign (& mut self , other : $ t ) {* self = Self :: new (self . value * (other as i64 ) , self . modulo ) ; } } impl std :: ops :: Div <$ t > for ModInt {type Output = Self ; fn div (self , other : $ t ) -> Self {let inv = mod_inv (other as i64 , self . modulo ) ; Self :: new (self . value * inv , self . modulo ) } } impl std :: ops :: DivAssign <$ t > for ModInt {fn div_assign (& mut self , other : $ t ) {let inv = mod_inv (other as i64 , self . modulo ) ; * self = Self :: new (self . value * inv , self . modulo ) ; } } ) * ) }
impl_mod_int ! (u8 i8 u16 i16 u32 i32 u64 i64 usize isize );

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

/// 二項係数を mod のもとで求める
/// cf. [よくやる二項係数 (nCk mod. p)、逆元 (a^-1 mod. p) の求め方 - けんちょんの競プロ精進記録](http://drken1215.hatenablog.com/entry/2018/06/08/210000)
pub struct Comb {
    max_size: usize,
    modulo: usize,
    factorical_table: Vec<usize>,
    factorical_inverse_table: Vec<usize>,
    inverse_table: Vec<usize>,
}
impl Comb {
    pub fn new(max_size: usize, modulo: usize) -> Self {
        let max_size = std::cmp::max(10, max_size);
        assert!(max_size <= 10_000_000);
        let mut factorical_table = vec![0; max_size];
        let mut factorical_inverse_table = vec![0; max_size];
        let mut inverse_table = vec![0; max_size];
        factorical_table[0] = 1;
        factorical_table[1] = 1;
        factorical_inverse_table[0] = 1;
        factorical_inverse_table[1] = 1;
        inverse_table[1] = 1;
        for i in 2..max_size {
            factorical_table[i] = factorical_table[i - 1] * i % modulo;
            inverse_table[i] = modulo - inverse_table[modulo % i] * (modulo / i) % modulo;
            factorical_inverse_table[i] =
                factorical_inverse_table[i - 1] * inverse_table[i] % modulo;
        }
        Self {
            max_size,
            modulo,
            factorical_table,
            factorical_inverse_table,
            inverse_table,
        }
    }
    pub fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.factorical_table[n]
                * (self.factorical_inverse_table[k] * self.factorical_inverse_table[n - k]
                    % self.modulo)
                % self.modulo
        }
    }
}
