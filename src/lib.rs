#![feature(unboxed_closures)]

// Map a vector/list based on a mapping function
// @param map_fn: Mapping function
// @param vect: The vector to map
// @returns: A mapped vector
pub fn map<F, A, B> (mut map_fn: F, vect: Vec<A>) -> Vec<B> 
    where F: FnMut(A) -> B,
    A: Clone {
    let mut list = vec![];

    for v in vect {
        let mut value = map_fn(v.clone());
        list.push(value);
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_simple_list() {
        let list = vec![1, 2, 3];
        let expected_list = vec![2, 3, 4];
        assert_eq!(map(plus_one, list), expected_list);
    }

    #[test]
    fn map_empty_list() {
        let list = vec![];
        let expected_list = vec![];
        assert_eq!(map(plus_one, list), expected_list);
    }

    fn plus_one(n: i32) -> i32 {
        match n {
            x if x > 0 => n + 1,
            _ => 1
        }
    }
}