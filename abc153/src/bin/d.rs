use libprocon::*;

fn main() {
    let h = read!(usize);
    let mut x = 0usize;
    loop {
        let lower = 1 << x;
        let upper = 1 << (x + 1);
        if lower <= h && h < upper {
            break;
        }
        x += 1;
    }
    let mut ans = 1;
    for i in 1..=x {
        ans = ans + 2usize.pow(i as u32);
    }
    println!("{}", ans);
}
