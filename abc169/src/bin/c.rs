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
macro_rules !get {(@inner [$src :expr ] chars ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .chars () .collect ::<Vec <char >>() } } ;(@inner [$src :expr ] usize1 ) =>{{get !(@inner [$src ] usize ) -1 } } ;(@inner [$src :expr ] [usize1 ] ) =>{{get !(@inner [$src ] [usize ] ) .into_iter () .map (|v |v -1 ) .collect ::<Vec <usize >>() } } ;(@inner [$src :expr ] [[usize1 ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [usize1 ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[chars ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [chars ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [($($tt :tt ) ,*) ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] ($($tt ) ,*) ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] ($($tt :tt ) ,*) ) =>{{let mut buf :String =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;let mut iter =buf .split_whitespace () ;($(get !(@inner_elem_parse [$tt ] iter .next () .unwrap () ) ,) *) } } ;(@inner [$src :expr ] [$t :ty ] ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .map (|t |t .parse ::<$t >() .unwrap () ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[$t :ty ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [$t :ty ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] $t :ty ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .next () .unwrap () .parse ::<$t >() .unwrap () } } ;(@inner_elem_parse [usize1 ] $elem :expr ) =>{{get !(@inner_elem_parse [usize ] $elem ) -1 } } ;(@inner_elem_parse [$t :ty ] $elem :expr ) =>{{$elem .parse ::<$t >() .unwrap () } } ;($tt :tt ) =>{{use std ::io ::BufRead ;let get_stdin =std ::io ::stdin () ;let mut locked_stdin =get_stdin .lock () ;get !(@inner [&mut locked_stdin ] $tt ) } } ;}
macro_rules !debug {($($a :expr ) ,*$(,) *) =>{#[cfg (debug_assertions ) ] eprintln !(concat !($("| " ,stringify !($a ) ,"={:?} " ) ,*,"|" ) ,$(&$a ) ,*) ;} ;}
macro_rules !echo {($($a :expr ) ,*) =>{let mut s =Vec ::new () ;$(s .push (format !("{}" ,$a ) ) ;) *println !("{}" ,s .join (" " ) ) ;} }
#[macro_export]
macro_rules !chmin {($base :expr ,$($cmps :expr ) ,+$(,) *) =>{{let cmp_min =min !($($cmps ) ,+) ;if $base >cmp_min {$base =cmp_min ;true } else {false } } } ;}
#[macro_export]
macro_rules !chmax {($base :expr ,$($cmps :expr ) ,+$(,) *) =>{{let cmp_max =max !($($cmps ) ,+) ;if $base <cmp_max {$base =cmp_max ;true } else {false } } } ;}
#[macro_export]
macro_rules !min {($a :expr $(,) *) =>{{$a } } ;($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::min ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::min ($a ,min !($($rest ) ,+) ) } } ;}
#[macro_export]
macro_rules !max {($a :expr $(,) *) =>{{$a } } ;($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::max ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::max ($a ,max !($($rest ) ,+) ) } } ;}
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
    let (a, b) = get!((i64, f64));
    let mut x = format!("{:.2}", b);
    debug!(x);
    x = x.replace(".", "");
    let b = x.parse::<i64>().unwrap();
    debug!(b);
    let mut r = Rational::new(b, 100);
    r *= a;
    echo!(r.floor().to_integer());
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
macro_rules !impl_int_for_numerics {($($t :ty ) *) =>{$(impl Int for $t {fn zero () ->Self {0 } fn one () ->Self {1 } fn next (self ) ->Self {self +Self ::one () } fn prev (self ) ->Self {self -Self ::one () } } ) *} }
impl_int_for_numerics !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
#[derive(Clone, Copy)]
pub struct Rational<T: Int> {
    /// Numerator
    numerator: T,
    /// Denomitor
    denomitor: T,
}
impl<T> std::fmt::Display for Rational<T>
where
    T: Int + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.numerator, self.denomitor)
    }
}
impl<T> std::fmt::Debug for Rational<T>
where
    T: Int + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.numerator, self.denomitor)
    }
}
impl<T: Int> Rational<T> {
    pub fn new(numerator: T, denomitor: T) -> Rational<T> {
        let mut r = Rational {
            numerator,
            denomitor,
        };
        r.reduce();
        r
    }
    pub fn from_integer(t: T) -> Rational<T> {
        Rational::new(t, T::one())
    }
    pub fn to_integer(&self) -> T {
        self.truncate().numerator
    }
    /// Round value towards zero.
    pub fn truncate(&self) -> Rational<T> {
        Rational::from_integer(self.numerator / self.denomitor)
    }
    /// Round value towards minus infinity.
    pub fn floor(&self) -> Rational<T> {
        if *self < T::zero() {
            Rational::from_integer((self.numerator - self.denomitor + T::one()) / self.denomitor)
        } else {
            Rational::from_integer(self.numerator / self.denomitor)
        }
    }
    /// Round value towards plus infinity.
    pub fn ceil(&self) -> Rational<T> {
        if *self < T::zero() {
            Rational::from_integer(self.numerator / self.denomitor)
        } else {
            Rational::from_integer((self.numerator + self.denomitor - T::one()) / self.denomitor)
        }
    }
    /// Puts self into lowest terms, with denomitor > 0.
    fn reduce(&mut self) {
        assert!(!self.denomitor.is_zero());
        if self.numerator.is_zero() {
            self.denomitor = T::one();
            return;
        }
        if self.numerator == self.denomitor {
            self.numerator = T::one();
            self.denomitor = T::one();
            return;
        }
        let g = gcd(self.numerator, self.denomitor);
        self.numerator = self.numerator / g;
        self.denomitor = self.denomitor / g;
        if self.denomitor < T::zero() {
            self.numerator = T::zero() - self.numerator;
            self.denomitor = T::zero() - self.denomitor;
        }
    }
}
impl<T> From<T> for Rational<T>
where
    T: Int,
{
    fn from(x: T) -> Self {
        Rational::from_integer(x)
    }
}
impl<T> From<(T, T)> for Rational<T>
where
    T: Int,
{
    fn from((numerator, denomitor): (T, T)) -> Self {
        Rational::new(numerator, denomitor)
    }
}
impl<T> PartialEq<Self> for Rational<T>
where
    T: Int,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl<T> Eq for Rational<T> where T: Int {}
impl<T> PartialEq<T> for Rational<T>
where
    T: Int,
{
    fn eq(&self, other: &T) -> bool {
        let other = Self::from_integer(*other);
        *self == other
    }
}
macro_rules !impl_partial_eq_with_rational_for_numerics {($($t :ty ) *) =>{$(impl PartialEq <Rational <$t >>for $t {fn eq (&self ,other :&Rational <$t >) ->bool {let r =Rational ::from_integer (*self ) ;r ==*other } } ) *} }
impl_partial_eq_with_rational_for_numerics !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
impl<T> Ord for Rational<T>
where
    T: Int,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.denomitor == other.denomitor {
            return if self.denomitor > T::zero() {
                self.numerator.cmp(&other.numerator)
            } else {
                self.numerator.cmp(&other.numerator).reverse()
            };
        }
        if self.numerator == other.numerator {
            return match self.numerator.cmp(&T::zero()) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => self.denomitor.cmp(&other.denomitor).reverse(),
                std::cmp::Ordering::Less => self.denomitor.cmp(&other.denomitor),
            };
        }
        let self_int = self.numerator.div_floor(&self.denomitor);
        let self_rem = self.numerator.mod_floor(&self.denomitor);
        let other_int = other.numerator.div_floor(&other.denomitor);
        let other_rem = other.numerator.mod_floor(&other.denomitor);
        match self_int.cmp(&other_int) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => match (self_rem.is_zero(), other_rem.is_zero()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => {
                    let self_new = Rational::new(self.denomitor, self_rem);
                    let other_new = Rational::new(other.denomitor, other_rem);
                    self_new.cmp(&other_new).reverse()
                }
            },
        }
    }
}
impl<T> PartialOrd<Self> for Rational<T>
where
    T: Int,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> PartialOrd<T> for Rational<T>
where
    T: Int,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        let other = Self::from_integer(*other);
        Some(self.cmp(&other))
    }
}
macro_rules !impl_partial_ord_with_rational_for_numerics {($($t :ty ) *) =>{$(impl PartialOrd <Rational <$t >>for $t {fn partial_cmp (&self ,other :&Rational <$t >) ->Option <std ::cmp ::Ordering >{let r =Rational ::from_integer (*self ) ;Some (r .cmp (other ) ) } } ) *} }
impl_partial_ord_with_rational_for_numerics !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
impl<T> std::ops::Add<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let denom_gcd = gcd(self.denomitor, rhs.denomitor);
        let denom_lcm = (self.denomitor.mul(rhs.denomitor)).div(denom_gcd);
        let self_mul = (rhs.denomitor).div(denom_gcd);
        let rhs_mul = (self.denomitor).div(denom_gcd);
        let numer = self.numerator.mul(self_mul).add(rhs.numerator.mul(rhs_mul));
        Rational::new(numer, denom_lcm)
    }
}
impl<T> std::ops::Add<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self.add(rhs)
    }
}
impl<T> std::ops::AddAssign<Self> for Rational<T>
where
    T: Int,
{
    fn add_assign(&mut self, other: Self) {
        let add = std::ops::Add::<Self>::add(*self, other);
        self.numerator = add.numerator;
        self.denomitor = add.denomitor;
    }
}
impl<T> std::ops::AddAssign<T> for Rational<T>
where
    T: Int,
{
    fn add_assign(&mut self, other: T) {
        let add = std::ops::Add::<T>::add(*self, other);
        self.numerator = add.numerator;
        self.denomitor = add.denomitor;
    }
}
impl<T> std::ops::Sub<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let denom_gcd = gcd(self.denomitor, rhs.denomitor);
        let denom_lcm = self.denomitor.mul(rhs.denomitor).div(denom_gcd);
        let self_mul = rhs.denomitor.div(denom_gcd);
        let rhs_mul = self.denomitor.div(denom_gcd);
        let numer = self.numerator.mul(self_mul).sub(rhs.numerator.mul(rhs_mul));
        Rational::new(numer, denom_lcm)
    }
}
impl<T> std::ops::Sub<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self - rhs
    }
}
impl<T> std::ops::SubAssign<Self> for Rational<T>
where
    T: Int,
{
    fn sub_assign(&mut self, other: Self) {
        let sub = *self - other;
        self.numerator = sub.numerator;
        self.denomitor = sub.denomitor;
    }
}
impl<T> std::ops::SubAssign<T> for Rational<T>
where
    T: Int,
{
    fn sub_assign(&mut self, other: T) {
        let sub = *self - other;
        self.numerator = sub.numerator;
        self.denomitor = sub.denomitor;
    }
}
impl<T> std::ops::Mul<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let gcd1 = gcd(self.numerator, rhs.denomitor);
        let gcd2 = gcd(self.denomitor, rhs.numerator);
        let num1 = self.numerator.div(gcd1);
        let den1 = self.denomitor.div(gcd2);
        let num2 = rhs.numerator.div(gcd2);
        let den2 = rhs.denomitor.div(gcd1);
        Self::new(num1 * num2, den1 * den2)
    }
}
impl<T> std::ops::Mul<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self * rhs
    }
}
impl<T> std::ops::MulAssign<Self> for Rational<T>
where
    T: Int,
{
    fn mul_assign(&mut self, rhs: Self) {
        let mul = *self * rhs;
        self.numerator = mul.numerator;
        self.denomitor = mul.denomitor;
    }
}
impl<T> std::ops::MulAssign<T> for Rational<T>
where
    T: Int,
{
    fn mul_assign(&mut self, rhs: T) {
        let mul = *self * rhs;
        self.numerator = mul.numerator;
        self.denomitor = mul.denomitor;
    }
}
impl<T> std::ops::Div<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = Self::new(rhs.denomitor, rhs.numerator);
        std::ops::Mul::<Rational<T>>::mul(self, rhs_inv)
    }
}
impl<T> std::ops::Div<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs_inv = Self::new(T::one(), rhs);
        std::ops::Mul::<Rational<T>>::mul(self, rhs_inv)
    }
}
impl<T> std::ops::DivAssign<Self> for Rational<T>
where
    T: Int,
{
    fn div_assign(&mut self, rhs: Self) {
        let div = *self / rhs;
        self.numerator = div.numerator;
        self.denomitor = div.denomitor;
    }
}
impl<T> std::ops::DivAssign<T> for Rational<T>
where
    T: Int,
{
    fn div_assign(&mut self, rhs: T) {
        let div = *self / rhs;
        self.numerator = div.numerator;
        self.denomitor = div.denomitor;
    }
}
macro_rules !impl_ops_for_numerics {($($t :ty ) *) =>{$(impl std ::ops ::Add <Rational <$t >>for $t {type Output =Rational <$t >;fn add (self ,rhs :Rational <$t >) ->Self ::Output {Rational ::from_integer (self ) .add (rhs ) } } impl std ::ops ::Sub <Rational <$t >>for $t {type Output =Rational <$t >;fn sub (self ,rhs :Rational <$t >) ->Self ::Output {Rational ::from_integer (self ) .sub (rhs ) } } impl std ::ops ::Mul <Rational <$t >>for $t {type Output =Rational <$t >;fn mul (self ,rhs :Rational <$t >) ->Self ::Output {Rational ::from_integer (self ) .mul (rhs ) } } impl std ::ops ::Div <Rational <$t >>for $t {type Output =Rational <$t >;fn div (self ,rhs :Rational <$t >) ->Self ::Output {Rational ::from_integer (self ) .div (rhs ) } } ) *} }
impl_ops_for_numerics !(u8 i8 u16 i16 u32 i32 u64 i64 usize isize );
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
