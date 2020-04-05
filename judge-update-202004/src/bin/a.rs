use proconio::input;

fn main() {
    input! {
        s: i32,
        l: i32,
        r: i32,
    }
    if l <= s && s <= r {
        println!("{}", s);
    } else if s < l {
        println!("{}", l);
    } else {
        println!("{}", r);
    }
}
