use libprocon::*;

fn main() {
    input! {
        x: usize,
    }
    let primes = 200000usize.lower_primes();
    for p in primes {
        if p >= x {
            println!("{}", p);
            return;
        }
    }
}
