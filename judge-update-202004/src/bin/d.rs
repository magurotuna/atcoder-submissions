use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [i64; N],
        S: [i64; Q],
    }
    let mut acc_gcd = vec![0; N];
    acc_gcd[0] = A[0];
    for i in 1..A.len() {
        acc_gcd[i] = gcd(acc_gcd[i - 1], A[i]);
    }
    let mut ans = Vec::new();
    for i in 0..Q {
        let x = S[i];
        let j0 = {
            let mut ok = acc_gcd.len() as i64;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if gcd(x, acc_gcd[mid as usize]) == 1 {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize + 1
        };
        if j0 > N {
            ans.push(gcd(x, acc_gcd[acc_gcd.len() - 1]));
        } else {
            ans.push(j0 as i64);
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
