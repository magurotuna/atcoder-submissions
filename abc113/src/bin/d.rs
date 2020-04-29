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
        H: usize,
        W: usize,
        K: usize,
    }
    let mut dp = vec![vec![ModInt::new(0, 1_000_000_007); 10]; 110];
    dp[0][0] = ModInt::new(1, 1_000_000_007);
    for h in 0..H {
        for i in 0..(1 << (W - 1)) {
            if !is_ok(i) {
                //dbg!(format!("{:b}", i));
                continue;
            }
            for w in 0..W {
                if w + 1 <= W - 1 && (1 << w) & i != 0 {
                    let tmp = dp[h][w + 1];
                    dp[h + 1][w] += tmp;
                } else if w >= 1 && (1 << (w - 1)) & i != 0 {
                    let tmp = dp[h][w - 1];
                    dp[h + 1][w] += tmp;
                } else {
                    let tmp = dp[h][w];
                    dp[h + 1][w] += tmp;
                }
            }
        }
    }
    println!("{}", dp[H][K - 1]);
}

fn is_ok(x: usize) -> bool {
    for w in format!("{:b}", x).chars().collect::<Vec<_>>().windows(2) {
        if w[0] == '1' && w[0] == w[1] {
            return false;
        }
    }
    true
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
            modulo: modulo,
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
