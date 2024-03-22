#![allow(unused_variables)]
fn main() {
    let n = -1;
    let m = 2;
    let sum = get_sum(n,m);
    println!("Sum is: {}", sum);
}

fn get_sum(n: i32, m: i32) -> i32 {
    return n+m;
}

#[test]
fn test_get_sum() {
    assert_eq!(get_sum(1,2), 3);
    assert_eq!(get_sum(3,4), 7);
}

#[test]
fn test_get_sum_negative() {
    assert_eq!(get_sum(-1,2), 1);
    assert_eq!(get_sum(-3,-4), -7);
}