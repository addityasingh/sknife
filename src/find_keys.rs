#![feature(unboxed_closures)]
#![crate_name = "sknife"]
use std::collections::HashMap;

/// Finds list of all keys in a Hashmap satisfying a predicate
///
/// # Arguments
///
/// * `predicate` - The predicate function
/// * `obj` - A hashmap/object to find the keys in
///
/// # Example
///
/// ```
/// use sknife::find_keys::find_keys;
/// use std::collections::HashMap;
/// let obj = HashMap::new();
/// let greater_than_one = |x: i32| x > 1;
/// find_keys(greater_than_one, obj);
/// 
/// ```
/// 
/// # Result
/// ```
/// vec![String::from("Three"), String::from("Four")];
/// ```
pub fn find_keys<V, F> (mut predicate: F, obj: HashMap<String, V>) -> Vec<String>
    where F: FnMut(V) -> bool,
    V: Clone {
        let mut list: Vec<String> = vec![];
        for (k, v) in obj {
            match predicate(v.clone()) {
                true => {
                    list.push(k);
                },
                false => continue
            }
        }

        list
}

mod tests {
    use super::*;

    #[test]
    fn find_keys_hash() {
        let mut obj = HashMap::new();
        obj.insert(String::from("One"), 1);
        obj.insert(String::from("Three"), 3);
        obj.insert(String::from("Four"), 4);
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            find_keys(greater_than_one, obj), 
            vec![
                String::from("Four"),
                String::from("Three")
            ]
        );
    }

    #[test]
    fn find_keys_empty_hash() {
        let mut obj = HashMap::new();
        let greater_than_one = |x: i32| x > 1;
        let expected_result: Vec<String> = vec![];
        assert_eq!(
            find_keys(greater_than_one, obj), 
            expected_result
        );
    }
}