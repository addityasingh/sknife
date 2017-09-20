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
/// any(greater_than_one, list.into_iter());
/// 
/// ```
/// 
/// # Result
/// ```
/// true;
/// ```
pub fn any<T, F, I> (mut predicate: F, list: I) -> bool
    where F: FnMut(T) -> bool,
    I: Iterator<Item=T>,
    T: Clone {
        let mut result: bool = false;

        for v in list {
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
            any(greater_than_one, list.into_iter()), 
            false
        );
    }

    #[test]
    fn any_list() {
        let mut list = vec![1, 2, 3, 4];
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            any(greater_than_one, list.into_iter()), 
            true
        );
    }
}