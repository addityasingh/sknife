The results of benchmark test on the funcitons against native iterator equivalensts are:

# System condition
- MacBook Pro (Retina, 13-inch, Early 2015)

- 2.7 GHz Intel Core i5
- 16 GB 1867 MHz DDR3

# Methods

## reduce

```rust
    b.iter(|| reduce(&fact, list.as_mut_slice(), 1));

    b.iter(|| list.iter().fold(1, &fact));
```
```bash

running 2 tests
test tests::bench_reduce_func   ... bench:          26 ns/iter (+/- 10)
test tests::bench_reduce_method ... bench:          25 ns/iter (+/- 7)
```