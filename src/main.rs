extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];
    println!("{}", vec.par_iter().all(|n| (n % 2) == 0));
    // print!("{}", vec.par_iter().all(|n| &n <= 8));

    vec.push(9);

    println!("{}",vec.par_iter().all(|n|(n%2)==0));
    println!("{}", vec.par_iter().all(|n| *n <=8));
}
