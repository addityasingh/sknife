extern crate sknife;

use sknife::collection::find;

#[test]
fn find_greater_than_five () {
    let mut list = vec![1, 3, 4, 7, 3, 9];
    let greater_than_five = |x: i32| x > 5;

    assert_eq!(
        find(greater_than_five, list.to_vec()),
        Some(7)
    );
}