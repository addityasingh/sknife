#![feature(test)]

extern crate sknife;
extern crate test;

use sknife::collection::reduce;

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_reduce_func(b: &mut Bencher) {
        let mut list: Vec<i32> = (1..50).collect();
        let mut fact = |acc: i32, x| acc * x;
        b.iter(|| reduce(&fact, list.as_mut_slice(), 1));
    }

    #[bench]
    fn bench_reduce_method(b: &mut Bencher) {
        let mut list: Vec<i32> = (1..50).collect();
        let mut fact = |acc: i32, x| acc * x;
        b.iter(|| list.iter().fold(1, &fact));
    }
}