extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![6,2,1,9,3,8,11];
    let f1 = vec.par_iter().find_any(|&&n| n==9);
    let f2 = vec.par_iter().find_any(|&&n| n % 2 ==0 && n > 6);
    let f3 = vec.par_iter().find_any(|&&n| n >8);
    println!("{:?}", f1);
    println!("{:?}", f2);
    println!("{:?}", f3);
    
}
