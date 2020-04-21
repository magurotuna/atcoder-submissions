#!/bin/bash

set -eu

if [ $# -ne 1 ]; then
  echo "Specify one argument."
  exit 1
fi

cargo init $1
cd $1
git init
mkdir src/bin

cat <<'EOF' >> Cargo.toml
libprocon = { git = "https://github.com/magurotuna/libprocon.git" }
num = "0.2.1"
num-bigint = "0.2.6"
num-complex = "0.2.4"
num-integer = "0.1.42"
num-iter = "0.1.40"
num-rational = "0.2.4"
num-traits = "0.2.11"
num-derive = "0.3.0"
ndarray = "0.13.0"
nalgebra = "0.20.0"
alga = "0.9.3"
libm = "0.2.1"
rand = { version = "0.7.3", features = ["small_rng"] }
getrandom = "0.1.14"
rand_chacha = "0.2.2"
rand_core = "0.5.1"
rand_hc = "0.2.0"
rand_pcg = "0.2.1"
rand_distr = "0.2.2"
petgraph = "0.5.0"
indexmap = "1.3.2"
regex = "1.3.6"
lazy_static = "1.4.0"
ordered-float = "1.0.2"
ascii = "1.0.0"
permutohedron = "0.2.4"
superslice = "1.0.0"
itertools = "0.9.0"
itertools-num = "0.1.3"
maplit = "1.0.2"
either = "1.5.3"
im-rc = "14.3.0"
fixedbitset = "0.2.0"
bitset-fixed = "0.1.0"
proconio = { version = "0.3.6", features = ["derive"] }
text_io = "0.1.8"
whiteread = "0.5.0"
rustc-hash = "1.1.0"
smallvec = "1.2.0"
EOF

for f in a.rs b.rs c.rs d.rs e.rs f.rs
do
  cat <<'EOF' > src/bin/$f
#![allow(unused_imports)]

use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars, Usize1};

#[fastout]
fn main() {
    input! {
        r: f64,
    }
    println!("{}", 2.0 * r * std::f64::consts::PI);
}
EOF
done