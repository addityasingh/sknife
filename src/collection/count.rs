#![crate_name = "sknife"]

/// Get the count of elements in the list
///
/// # Arguments
///
/// * `list` - A list of elements to find in
///
/// # Example
///
/// ```
/// use sknife::collection::count;
/// let list = vec![1, 2, 3, 4];
/// count(list.into_iter());
/// 
/// ```
/// 
/// # Result
/// ```
/// 4;
/// ```
pub fn count<T, I> (list: I) -> usize 
    where I: Iterator<Item=T> {
        list.count()
}

mod tests {
    use super::*;

    #[test]
    fn count_empty_list() {
        let mut list: Vec<i32> = vec![];
        assert_eq!(count(list.into_iter()), 0);
    }

    #[test]
    fn count_of_list() {
        let mut list = vec![1, 2, 3, 4];
        assert_eq!(count(list.into_iter()), 4);
    }
}