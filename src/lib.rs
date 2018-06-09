#![feature(test)]
extern crate test;

fn vec_access<T: Copy>(a: &[T]) -> T {
    a[10000]
}

use test::Bencher;

#[bench]
fn bench_vec_access(b: &mut test::Bencher) {
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| vec_access(&a))
}

#[bench]
fn bench_vec_insert(b: &mut test::Bencher) {
    // O(n)
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| a.insert(0, 1))
}

#[bench]
fn bench_vec_push(b: &mut test::Bencher) {
    // O(1)
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| a.push(1))
}
