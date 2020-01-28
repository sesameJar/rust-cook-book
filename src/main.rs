extern crate rand;

use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..7);
    loop {
        // println!("{:?}", &mut rng);
        let throw = die.sample(&mut rng);
        println!("Roll the dice : {}", throw);
        if throw == 6 {
            break;
        }
    }
}
