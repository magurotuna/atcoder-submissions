use libprocon::*;

enum CharIndex {
    Dummy,
    A,
    C,
    G,
    T,
}

fn main() {
    input! {
        N: usize,
    }
    let MOD = 1_000_000_007;

    let mut dp = vec![vec![vec![vec![0; 5]; 5]; 5]; N + 1];
    dp[0][0][0][0] = 1;
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                // AGC, ACG, GAC はだめ
                if i == 1 && j == 3 && k == 2
                    || i == 1 && j == 2 && k == 3
                    || i == 3 && j == 1 && k == 2
                {
                    continue;
                }
                dp[3][i][j][k] = 1;
            }
        }
    }

    for n in 4..=N {
        // AGC, ACG, GAC, A*GC, AG*C がだめなパターン
        // これらNGパターンに遷移しないように遷移式を構築していく
        for i in 1..5 {
            for j in 1..5 {
                for k in 1..5 {
                    for any in 1..5 {
                        // AGC
                        let agc = i == 1 && j == 3 && k == 2;
                        // ACG
                        let acg = i == 1 && j == 2 && k == 3;
                        // GAC
                        let gac = i == 3 && j == 1 && k == 2;
                        // A*GC
                        let a_gc = any == 1 && j == 3 && k == 2;
                        // AG*C
                        let ag_c = any == 1 && i == 3 && k == 2;
                        if agc || acg || gac || a_gc || ag_c {
                            continue;
                        }
                        dp[n][i][j][k] += dp[n - 1][any][i][j];
                        dp[n][i][j][k] %= MOD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 1..5 {
        for j in 1..5 {
            for k in 1..5 {
                ans += dp[N][i][j][k];
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}

enum Prev2Chars {
    AA,
    AC,
    AG,
    AT,
    CA,
    CC,
    CG,
    CT,
    GA,
    GC,
    GG,
    GT,
    TA,
    TC,
    TG,
    TT,
}

fn my_solution() {
    input! {
        N: usize,
    }
    let MOD = 1_000_000_007;

    let mut dp = vec![vec![ModInt::new(0, MOD); 16]; 100];
    for i in 0..16 {
        dp[1][i] = dp[0][i] + 1;
    }

    let mut bad_cnt = ModInt::new(0, MOD);

    for n in 2..N {
        // AA
        let aa = dp[n - 1][Prev2Chars::AA as usize];
        dp[n][Prev2Chars::AA as usize] += aa;
        dp[n][Prev2Chars::AC as usize] += aa;
        dp[n][Prev2Chars::AG as usize] += aa;
        dp[n][Prev2Chars::AT as usize] += aa;
        // AC
        let ac = dp[n - 1][Prev2Chars::AC as usize];
        dp[n][Prev2Chars::CA as usize] += ac;
        dp[n][Prev2Chars::CC as usize] += ac;
        dp[n][Prev2Chars::CG as usize] += ac; // これがアウト
        dp[n][Prev2Chars::CT as usize] += ac;
        bad_cnt += ac;
        // AG
        let ag = dp[n - 1][Prev2Chars::AG as usize];
        dp[n][Prev2Chars::GA as usize] += ag;
        dp[n][Prev2Chars::GC as usize] += ag; // これがアウト
        dp[n][Prev2Chars::GG as usize] += ag;
        dp[n][Prev2Chars::GT as usize] += ag;
        bad_cnt += ag;
        // AT
        let at = dp[n - 1][Prev2Chars::AT as usize];
        dp[n][Prev2Chars::TA as usize] += at;
        dp[n][Prev2Chars::TC as usize] += at;
        dp[n][Prev2Chars::TG as usize] += at;
        dp[n][Prev2Chars::TT as usize] += at;

        // CA
        let ca = dp[n - 1][Prev2Chars::CA as usize];
        dp[n][Prev2Chars::AA as usize] += ca;
        dp[n][Prev2Chars::AC as usize] += ca;
        dp[n][Prev2Chars::AG as usize] += ca;
        dp[n][Prev2Chars::AT as usize] += ca;
        // CC
        let cc = dp[n - 1][Prev2Chars::CC as usize];
        dp[n][Prev2Chars::CA as usize] += cc;
        dp[n][Prev2Chars::CC as usize] += cc;
        dp[n][Prev2Chars::CG as usize] += cc;
        dp[n][Prev2Chars::CT as usize] += cc;
        // CG
        let cg = dp[n - 1][Prev2Chars::CG as usize];
        dp[n][Prev2Chars::GA as usize] += cg;
        dp[n][Prev2Chars::GC as usize] += cg;
        dp[n][Prev2Chars::GG as usize] += cg;
        dp[n][Prev2Chars::GT as usize] += cg;
        // CT
        let ct = dp[n - 1][Prev2Chars::CT as usize];
        dp[n][Prev2Chars::TA as usize] += ct;
        dp[n][Prev2Chars::TC as usize] += ct;
        dp[n][Prev2Chars::TG as usize] += ct;
        dp[n][Prev2Chars::TT as usize] += ct;

        // GA
        let ga = dp[n - 1][Prev2Chars::GA as usize];
        dp[n][Prev2Chars::AA as usize] += ga;
        dp[n][Prev2Chars::AC as usize] += ga; // これがアウト
        dp[n][Prev2Chars::AG as usize] += ga;
        dp[n][Prev2Chars::AT as usize] += ga;
        bad_cnt += ga;
        // GC
        let gc = dp[n - 1][Prev2Chars::GC as usize];
        dp[n][Prev2Chars::CA as usize] += gc;
        dp[n][Prev2Chars::CC as usize] += gc;
        dp[n][Prev2Chars::CG as usize] += gc;
        dp[n][Prev2Chars::CT as usize] += gc;
        // GG
        let gg = dp[n - 1][Prev2Chars::GG as usize];
        dp[n][Prev2Chars::GA as usize] += gg;
        dp[n][Prev2Chars::GC as usize] += gg;
        dp[n][Prev2Chars::GG as usize] += gg;
        dp[n][Prev2Chars::GT as usize] += gg;
        // GT
        let gt = dp[n - 1][Prev2Chars::GT as usize];
        dp[n][Prev2Chars::TA as usize] += gt;
        dp[n][Prev2Chars::TC as usize] += gt;
        dp[n][Prev2Chars::TG as usize] += gt;
        dp[n][Prev2Chars::TT as usize] += gt;

        // TA
        let ta = dp[n - 1][Prev2Chars::TA as usize];
        dp[n][Prev2Chars::AA as usize] += ta;
        dp[n][Prev2Chars::AC as usize] += ta;
        dp[n][Prev2Chars::AG as usize] += ta;
        dp[n][Prev2Chars::AT as usize] += ta;
        // TC
        let tc = dp[n - 1][Prev2Chars::TC as usize];
        dp[n][Prev2Chars::CA as usize] += tc;
        dp[n][Prev2Chars::CC as usize] += tc;
        dp[n][Prev2Chars::CG as usize] += tc;
        dp[n][Prev2Chars::CT as usize] += tc;
        // TG
        let tg = dp[n - 1][Prev2Chars::TG as usize];
        dp[n][Prev2Chars::GA as usize] += tg;
        dp[n][Prev2Chars::GC as usize] += tg;
        dp[n][Prev2Chars::GG as usize] += tg;
        dp[n][Prev2Chars::GT as usize] += tg;
        // TT
        let tt = dp[n - 1][Prev2Chars::TT as usize];
        dp[n][Prev2Chars::TA as usize] += tt;
        dp[n][Prev2Chars::TC as usize] += tt;
        dp[n][Prev2Chars::TG as usize] += tt;
        dp[n][Prev2Chars::TT as usize] += tt;

        if n == 3 {
            dbg!(aa, ac, ag, at, ca, cc, cg, ct, ga, gc, gg, gt, ta, tc, tg, tt);
        }
    }

    let mint = ModInt::new(mod_pow(4, N as i64, MOD), MOD);
    dbg!(bad_cnt, mint);
}

#[doc = " 累乗のmod"]
#[doc = " cf. https://github.com/hatoo/competitive-rust-snippets/blob/master/src/modulo.rs"]
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
#[doc = " mod m での a の逆元を求める"]
#[doc = " m と a が互いに素でなければならないことに注意"]
#[doc = " cf. [「1000000007 で割ったあまり」の求め方を総特集！ 〜 逆元から離散対数まで 〜 - Qiita](https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a)"]
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
#[derive(Debug, Clone, Copy)]
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

macro_rules! impl_mod_int {
    ( $( $t: ty )* ) => (
        $(
            impl std::cmp::PartialEq<$t> for ModInt {
                fn eq(&self, other: &$t) -> bool {
                    self.value == (*other as i64)
                }
            }

            impl std::ops::Add<$t> for ModInt {
                type Output = Self;

                fn add(self, other: $t) -> Self {
                    Self::new(self.value + (other as i64), self.modulo)
                }
            }

            impl std::ops::AddAssign<$t> for ModInt {
                fn add_assign(&mut self, other: $t) {
                    *self = Self::new(self.value + (other as i64), self.modulo);
                }
            }

            impl std::ops::Sub<$t> for ModInt {
                type Output = Self;

                fn sub(self, other: $t) -> Self {
                    Self::new(self.value - (other as i64), self.modulo)
                }
            }

            impl std::ops::SubAssign<$t> for ModInt {
                fn sub_assign(&mut self, other: $t) {
                    *self = Self::new(self.value - (other as i64), self.modulo);
                }
            }

            impl std::ops::Mul<$t> for ModInt {
                type Output = Self;

                fn mul(self, other: $t) -> Self {
                    Self::new(self.value * (other as i64), self.modulo)
                }
            }

            impl std::ops::MulAssign<$t> for ModInt {
                fn mul_assign(&mut self, other: $t) {
                    *self = Self::new(self.value * (other as i64), self.modulo);
                }
            }

            impl std::ops::Div<$t> for ModInt {
                type Output = Self;

                fn div(self, other: $t) -> Self {
                    let inv = mod_inv(other as i64, self.modulo);
                    Self::new(self.value * inv, self.modulo)
                }
            }

            impl std::ops::DivAssign<$t> for ModInt {
                fn div_assign(&mut self, other: $t) {
                    let inv = mod_inv(other as i64, self.modulo);
                    *self = Self::new(self.value * inv, self.modulo);
                }
            }
        )*
    )
}
impl_mod_int ! ( u8 i8 u16 i16 u32 i32 u64 i64 usize isize ) ;
