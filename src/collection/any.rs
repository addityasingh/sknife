#![crate_name = "sknife"]

/// Find if any element of list satisfies the predicate
///
/// # Arguments
///
/// * `predicate` - The predicate function
/// * `list` - A list of elements to find in
///
/// # Example
///
/// ```
/// use sknife::collection::any;
/// let list = vec![1, 2, 3, 4];
/// let greater_than_one = |x: i32| x > 1;
/// any(greater_than_one, list);
/// 
/// ```
/// 
/// # Result
/// ```
/// true;
/// ```
pub fn any<T, F> (mut predicate: F, list: Vec<T>) -> bool
    where F: FnMut(T) -> bool,
    T: Clone {
        let mut result: bool = false;
        for v in list.iter() {
            if predicate(v.clone()) {
                result = true;
                break;
            }
        }
        result
}

mod tests {
    use super::*;

    #[test]
    fn any_empty_list() {
        let mut list = vec![];
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            any(greater_than_one, list), 
            false
        );
    }

    #[test]
    fn any_list() {
        let mut list = vec![1, 2, 3, 4];
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            any(greater_than_one, list), 
            true
        );
    }
}