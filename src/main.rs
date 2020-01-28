extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u64 = rng.gen();
    println!("{}", n1);
    println!("{}", n2);
    println!("{}", rng.gen::<f64>());
    println!("{}", rng.gen::<i32>());
}
