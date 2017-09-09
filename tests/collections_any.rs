extern crate sknife;

use sknife::collection::any;

#[test]
fn is_any_mod_two () {
    let mut list = vec![1, 2, 3, 4];
    let mod_two = |x: i32| x % 2 == 0;
    assert_eq!(
        any(mod_two, list.to_vec()),
        true
    ); // true
}