use libprocon::*;

fn main() {
    input! {
        N: usize,
        M: i64,
        a: [i64; N],
    }

    let two_included = count_two(a[0]);
    let mut l = 1;
    for i in 0..N {
        if two_included != count_two(a[i]) {
            println!("0");
            return;
        }
        l = lcm(l, a[i]);
    }
    let min_l = l / 2;
    println!("{}", ((M / min_l) + 1) / 2);
}

fn count_two(n: i64) -> i64 {
    let mut n = n;
    let mut cnt = 0;
    while n % 2 == 0 {
        cnt += 1;
        n /= 2;
    }
    cnt
}

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
    fn sqrt_floor(self) -> Self;
}
macro_rules ! impl_int_for_numerics { ( $ ( $ t : ty ) * ) => ( $ ( impl Int for $ t { fn zero ( ) -> Self { 0 } fn one ( ) -> Self { 1 } fn next ( self ) -> Self { self + Self :: one ( ) } fn prev ( self ) -> Self { self - Self :: one ( ) } fn sqrt_floor ( self ) -> Self { if self < Self :: zero ( ) { return Self :: zero ( ) ; } let two = Self :: one ( ) . next ( ) ; let mut ok = Self :: zero ( ) ; let mut ng = self . next ( ) ; while ng - ok > 1 { let mid = ( ng + ok ) / two ; if mid * mid <= self { ok = mid ; } else { ng = mid ; } } ok } } ) * ) }
impl_int_for_numerics ! ( u8 i8 u16 i16 u32 i32 u64 i64 usize isize ) ;
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Int,
{
    a / gcd(a, b) * b
}
