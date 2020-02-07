extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![6,2,1,9,3,8,11];
    let f1 = vec.par_iter().find_any(|&&n| n==9);
    let f2 = vec.par_iter().find_any(|&&n| n % 2 ==0 && n > 6);
    let f3 = vec.par_iter().find_any(|&&n| n >8);
    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}
