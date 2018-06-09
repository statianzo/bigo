#[allow(unused_imports)]
extern crate test;
use test::Bencher;

pub fn brute_force_search<T: PartialEq>(arr: &[T], el: T) -> bool {
    for i in 0..arr.len() {
        if arr[i] == el {
            return true;
        }
    }
    return false;
}

pub fn binary_search<T: Ord>(arr: &[T], el: T) -> bool {
    use std::cmp::Ordering;
    let mut remaining = arr;
    loop {
        let middle = remaining.len() / 2;
        match (remaining, el.cmp(&remaining[middle])) {
            (_, Ordering::Equal) => return true,
            ([_], _) => return false,
            (_, Ordering::Less) => remaining = &remaining[..middle],
            (_, Ordering::Greater) => remaining = &remaining[middle..],
        };
    }
}
#[test]
fn test_brute_force_miss() {
    let vec: Vec<usize> = (0..100).collect();
    assert_eq!(brute_force_search(&vec, 101), false)
}

#[test]
fn test_brute_force_hit() {
    let vec: Vec<usize> = (0..100).collect();
    assert_eq!(brute_force_search(&vec, 42), true)
}

#[test]
fn test_binary_miss() {
    let vec: Vec<usize> = (0..100).collect();
    assert_eq!(binary_search(&vec, 101), false)
}

#[test]
fn test_binary_hit() {
    let vec: Vec<usize> = (0..100).collect();
    assert_eq!(binary_search(&vec, 42), true)
}

#[bench]
fn bench_brute_force(b: &mut test::Bencher) {
    let vec: Vec<usize> = (0..1000000).collect();
    b.iter(|| brute_force_search(&vec, 900000));
}

#[bench]
fn bench_binary_search(b: &mut test::Bencher) {
    let vec: Vec<usize> = (0..1000000).collect();
    b.iter(|| binary_search(&vec, 900000));
}
