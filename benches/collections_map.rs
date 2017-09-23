#![feature(test)]

extern crate sknife;
extern crate test;

use sknife::collection::map;

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_map_func(b: &mut Bencher) {
        let mut list = vec![1, 2, 3];
        let mut slice = list.as_mut_slice();
        let mut plus_one = |x: i32| x + 1;
        b.iter(|| map(&plus_one, &slice));
    }

    #[bench]
    fn bench_map_method(b: &mut Bencher) {
        let mut list = vec![1, 2, 3];
        let mut plus_one = |x| x + 1;
        b.iter(|| list.iter().map(&plus_one));
    }
}