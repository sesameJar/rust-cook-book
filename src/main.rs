extern crate rand;

use rand::distributions::{Distribution, Normal};

fn main() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0);
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2,9) distribution", v)
}
