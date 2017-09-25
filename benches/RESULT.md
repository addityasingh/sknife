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

## map

```rust
    b.iter(|| map(&plus_one, slice));

    b.iter(|| list.iter().map(&plus_one));
```
```bash

test tests::bench_map_func   ... bench:          32 ns/iter (+/- 10)
test tests::bench_map_method ... bench:           1 ns/iter (+/- 0)
```