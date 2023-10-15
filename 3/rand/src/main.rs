mod random;

use crate::random::{linear, xorshift};

fn main() {
    let mut seed:u32 = 1;

    let r1 = linear::rand(&mut seed);
    let r2 = xorshift::rand(&mut seed);

    println!("{}, {}", r1, r2);
}