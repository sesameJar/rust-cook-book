fn main() {
    let mut vec = vec![1.1, 5.32, 10.1222, 2.2, 15.6, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(vec == vec![1.1, 2.0, 2.2, 5.32, 10.1222, 15.6]);
}
