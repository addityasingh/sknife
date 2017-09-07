extern crate sknife;

use sknife::collection::{any, find, map};

fn main () {
    let mut list = vec![1, 2, 3, 4];
    let mod_two = |x: i32| x % 2 == 0;
    println!(
        "Is 'any' element in list {:?} mod two? {:?}", 
        &list, 
        any(mod_two, list.to_vec())
    ); // true

    list = vec![1, 3, 4, 7, 3, 9];
    let greater_than_five = |x: i32| x > 5;

    println!(
        "'find' element in list {:?} greater than 5: {:?}", 
        &list, 
        find(greater_than_five, list.to_vec())
    ); // Some(7)
}