use libprocon::*;

#[derive(Debug, Clone, Copy)]
struct Monster {
    position: usize,
    health: usize,
}

fn main() {
    let (N, D, A) = read!(usize, usize, usize);
    let mut monsters = Vec::with_capacity(N);
    for _ in 0..N {
        let (position, health) = read!(usize, usize);
        monsters.push(Monster { position, health });
    }
    // TODO
}
