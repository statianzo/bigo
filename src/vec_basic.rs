#[allow(unused_imports)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_vec_access(b: &mut Bencher) {
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| a[99999])
}

#[bench]
fn bench_vec_insert(b: &mut Bencher) {
    // O(n)
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| a.insert(0, 1))
}

#[bench]
fn bench_vec_push(b: &mut Bencher) {
    // O(1)
    let mut a: Vec<usize> = Vec::with_capacity(100000);
    let len = a.capacity();
    a.resize(len, 0);
    println!("{}", a.len());
    b.iter(|| a.push(1))
}
