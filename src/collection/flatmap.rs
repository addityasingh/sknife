#![crate_name = "sknife"]

/// Flatten and map on a list
///
/// # Arguments
///
/// * `f` - the map function
/// * `list` - A slice of elements to flatten and map
///
/// # Example
///
/// ```
/// use sknife::collection::flatmap;
/// let mut list: Vec<i32> = (1..4).collect();
/// let slice: &mut [i32] = list.as_mut_slice();
/// flatmap(slice, |x: &mut i32| vec![*x]);
/// 
/// ```
/// 
/// # Result
/// ```
/// vec![1, 2, 3];
/// ```
pub fn flatmap<A, F>(list: &mut [A], mut f: F) -> Vec<A> 
    where F: FnMut(&mut A) -> Vec<A> {
	let mut vec = Vec::new();

	for l in list {
		vec.extend(f(l))
	}
	vec
}

mod tests {
    use super::*;

    #[test]
    fn flatmap_empty_list() {
        let mut list: Vec<i32> = vec![];
        let slice: &mut [i32] = list.as_mut_slice();
        assert_eq!(
            flatmap(slice, |x: &mut i32| vec![*x]),
            vec![]
        );
    }

    #[test]
    fn flatmap_list() {
        let mut list: Vec<i32> = (1..4).collect();
        let slice: &mut [i32] = list.as_mut_slice();
        assert_eq!(
            flatmap(slice, |x: &mut i32| vec![*x]),
            vec![1, 2, 3]
        );
    }
}