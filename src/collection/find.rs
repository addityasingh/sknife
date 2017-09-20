#![crate_name = "sknife"]

/// Find first element of list that satisfies a predicate
///
/// # Arguments
///
/// * `predicate` - The predicate function
/// * `list` - A list of elements to find in
///
/// # Example
///
/// ```
/// use sknife::collection::find;
/// let list = vec![1, 2, 3, 4];
/// let greater_than_one = |x: i32| x > 1;
/// find(greater_than_one, list.into_iter());
/// 
/// ```
/// 
/// # Result
/// ```
/// 2;
/// ```
pub fn find<T, F, I> (mut predicate: F, list: I) -> Option<T>
    where F: FnMut(T) -> bool,
    I: Iterator<Item=T>,
    T: Clone {
        let mut result: Option<T> = None;
        for v in list {
            if predicate(v.clone()) {
                result = Some(v.clone());
                break;
            }
        }
        result
}

mod tests {
    use super::*;

    #[test]
    fn find_empty_list() {
        let mut list = vec![];
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            find(greater_than_one, list.into_iter()), 
            None
        );
    }

    #[test]
    fn find_in_list() {
        let mut list = vec![1, 2, 3, 4];
        let greater_than_one = |x: i32| x > 1;
        assert_eq!(
            find(greater_than_one, list.into_iter()), 
            Some(2)
        );
    }
}