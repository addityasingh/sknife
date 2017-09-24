#![feature(test)]

extern crate sknife;
extern crate test;

use sknife::collection::flatmap;

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_flatmap_func(b: &mut Bencher) {
        let mut list: Vec<i32> = (1..1000).collect();
        let mut slice: &mut [i32] = list.as_mut_slice();
        let mut plus_one = |x: &mut i32| vec![*x];
        b.iter(|| flatmap(slice, &plus_one));
    }

    #[bench]
    fn bench_flatmap_method(b: &mut Bencher) {
        let mut list = vec![1, 2, 3];
        let mut plus_one = |x| vec![x];
        b.iter(|| list.iter().flat_map(&plus_one));
    }
}