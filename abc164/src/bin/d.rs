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
#![allow(unused_imports, unused_macros, dead_code)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;
mod util {
    use std::fmt::Debug;
    use std::io::{stdin, stdout, BufWriter, StdoutLock};
    use std::str::FromStr;
    pub fn line() -> String {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }
    pub fn chars() -> Vec<char> {
        line().chars().collect()
    }
    pub fn gets<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    }
    pub fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
        let out = stdout();
        let writer = BufWriter::new(out.lock());
        f(writer)
    }
}
macro_rules !get {([$t :ty ] ) =>{{let mut line :String =String ::new () ;stdin () .read_line (&mut line ) .unwrap () ;line .split_whitespace () .map (|t |t .parse ::<$t >() .unwrap () ) .collect ::<Vec <_ >>() } } ;([$t :ty ] ;$n :expr ) =>{(0 ..$n ) .map (|_ |get !([$t ] ) ) .collect ::<Vec <_ >>() } ;($t :ty ) =>{{let mut line :String =String ::new () ;stdin () .read_line (&mut line ) .unwrap () ;line .trim () .parse ::<$t >() .unwrap () } } ;($($t :ty ) ,*) =>{{let mut line :String =String ::new () ;stdin () .read_line (&mut line ) .unwrap () ;let mut iter =line .split_whitespace () ;($(iter .next () .unwrap () .parse ::<$t >() .unwrap () ,) *) } } ;($t :ty ;$n :expr ) =>{(0 ..$n ) .map (|_ |get !($t ) ) .collect ::<Vec <_ >>() } ;($($t :ty ) ,*;$n :expr ) =>{(0 ..$n ) .map (|_ |get !($($t ) ,*) ) .collect ::<Vec <_ >>() } ;}
macro_rules !debug {($($a :expr ) ,*) =>{eprintln !(concat !($(stringify !($a ) ," = {:?}, " ) ,*) ,$($a ) ,*) ;} }
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
    let s: Vec<char> = get!(String).chars().collect();
    let mut a = Vec::with_capacity(s.len());
    a.push(0);
    let MOD = 2019;
    for i in (0..s.len()).rev() {
        if i == s.len() - 1 {
            a.push(c2i(s[i]));
        } else {
            let &last = a.last().unwrap();
            let idx = s.len() - 1 - i;
            let t = mod_pow(10, idx as i64, MOD as i64) as usize;
            a.push((last + t * c2i(s[i])) % MOD);
        }
    }
    let mut map = HashMap::new();
    for aa in a {
        *map.entry(aa).or_insert(0) += 1
    }
    let mut ans = 0;
    for &v in map.values() {
        if v >= 2 {
            ans += ((v - 1) * v) / 2;
        }
    }
    println!("{}", ans);
}

fn c2i(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
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
pub fn mod_inv(a: i64, m: i64) -> i64 {
    use std::mem::swap;
    let mut a = a;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u
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
macro_rules !impl_mod_int {($($t :ty ) *) =>($(impl std ::cmp ::PartialEq <$t >for ModInt {fn eq (&self ,other :&$t ) ->bool {self .value ==(*other as i64 ) } } impl std ::ops ::Add <$t >for ModInt {type Output =Self ;fn add (self ,other :$t ) ->Self {Self ::new (self .value +(other as i64 ) ,self .modulo ) } } impl std ::ops ::AddAssign <$t >for ModInt {fn add_assign (&mut self ,other :$t ) {*self =Self ::new (self .value +(other as i64 ) ,self .modulo ) ;} } impl std ::ops ::Sub <$t >for ModInt {type Output =Self ;fn sub (self ,other :$t ) ->Self {Self ::new (self .value -(other as i64 ) ,self .modulo ) } } impl std ::ops ::SubAssign <$t >for ModInt {fn sub_assign (&mut self ,other :$t ) {*self =Self ::new (self .value -(other as i64 ) ,self .modulo ) ;} } impl std ::ops ::Mul <$t >for ModInt {type Output =Self ;fn mul (self ,other :$t ) ->Self {Self ::new (self .value *(other as i64 ) ,self .modulo ) } } impl std ::ops ::MulAssign <$t >for ModInt {fn mul_assign (&mut self ,other :$t ) {*self =Self ::new (self .value *(other as i64 ) ,self .modulo ) ;} } impl std ::ops ::Div <$t >for ModInt {type Output =Self ;fn div (self ,other :$t ) ->Self {let inv =mod_inv (other as i64 ,self .modulo ) ;Self ::new (self .value *inv ,self .modulo ) } } impl std ::ops ::DivAssign <$t >for ModInt {fn div_assign (&mut self ,other :$t ) {let inv =mod_inv (other as i64 ,self .modulo ) ;*self =Self ::new (self .value *inv ,self .modulo ) ;} } ) *) }
impl_mod_int !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
