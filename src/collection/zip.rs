#![crate_name = "sknife"]

type ListOptions<T> = Vec<(T, T)>;

use std;
/// Zips 2 list into a list of tuples
///
/// # Arguments
///
/// * `first_list` - First list
/// * `second_list` - Second list
///
/// # Example
///
/// ```
/// use sknife::collection::zip;
/// let first_list = vec![Some(1), Some(2), Some(3)];
/// let second_list = vec![Some(1), Some(4), Some(9)];
/// zip(first_list, second_list);
/// 
/// ```
/// 
/// # Result
/// ```
/// vec![(Some(1), Some(1)), (Some(2), Some(4)), (Some(3), Some(9))];
/// ```
pub fn zip<T> (
    first_list: Vec<T>, 
    second_list: Vec<T>) -> ListOptions<T> {
    // If any in the list is None, ignore it
    // Create iterator from both the lists
    // loop into first iter and combine corresponding elements
    let mut iter_first = first_list.into_iter();
    let mut iter_second = second_list.into_iter();
    let mut result: ListOptions<T> = vec![];

    loop {
        match iter_first.next() {
            None => { break }
            Some(v) => {
                match iter_second.next() {
                    None => { break }
                    Some(w) => result.push((v, w))
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip_simple_lists() {
        let first_list = vec![Some(1), Some(2), Some(3)];
        let second_list = vec![Some(1), Some(4), Some(9)];
        let expected_list = vec![(Some(1), Some(1)), (Some(2), Some(4)), (Some(3), Some(9))];
        assert_eq!(
            zip(first_list, second_list), 
            expected_list
        );
    }

    #[test]
    fn zip_empty_lists() {
        let first_list: Vec<Option<i32>> = vec![None];
        let second_list: Vec<Option<i32>> = vec![None];
        let expected_list = vec![(None, None)];
        assert_eq!(
            zip(first_list, second_list), 
            expected_list
        );
    }

    #[test]
    fn zip_unequal_lists() {
        let first_list = vec![Some(1), Some(2), Some(3)];
        let second_list = vec![Some(1), Some(4), Some(9), Some(16)];
        let expected_list = vec![(Some(1), Some(1)), (Some(2), Some(4)), (Some(3), Some(9))];
        assert_eq!(
            zip(first_list, second_list), 
            expected_list
        );
    }
}