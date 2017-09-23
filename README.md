# sknife
Swiss knife for common rust functional utilities

[![Build Status](https://travis-ci.org/addityasingh/sknife.svg?branch=master)](https://travis-ci.org/addityasingh/sknife)
[![](http://meritbadge.herokuapp.com/sknife)](https://crates.io/crates/sknife)
[![Documentation](https://docs.rs/sknife/badge.svg)](https://docs.rs/sknife/)

# Reason behind this
Primarily this is intended at creating functional utilities for common methods on the `Iterator` and `HashMap`'s in `Rust`

# Getting started
```
cargo install
cargo run
```

# Benchmark test
In order to compare the relative performance of the functions against there corresponding native `Iteratot` methods, there are some benchmark tests in the `benches` folder. Run the command to see the becnhmark comparison

```rust
cargo bench
```

Some results of the tests are already present [here](https://github.com/addityasingh/sknife/blob/master/benches/RESULT.md)

# Contribution
Add utility function if you think might be a useful as a more function-based ** not functional ;) ** approach

## API
Checkout the documentation at [docs.rs](https://docs.rs/sknife/)

## License
MIT © [addityasingh](http://github.com/addityasingh)
