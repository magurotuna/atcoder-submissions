use libprocon::*;

fn main() {
    input! {
        N: usize,
        A: [usize; N+1],
        B: [usize; N],
    }
    let mut a = A;
    let mut b = B;
    let mut ans = 0;

    for i in 0..N {
        if b[i] > a[i] {
            ans += a[i];
            b[i] -= a[i];
            a[i] = 0;
        } else {
            ans += b[i];
            a[i] -= b[i];
            b[i] = 0;
        }

        if b[i] > a[i + 1] {
            ans += a[i + 1];
            b[i] -= a[i + 1];
            a[i + 1] = 0;
        } else {
            ans += b[i];
            a[i + 1] -= b[i];
            b[i] = 0;
        }
    }
    println!("{}", ans);
}
