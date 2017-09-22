
#![crate_name = "sknife"]

/// Reduce a vector/list using an accumulating function
///
/// # Arguments
///
/// * `vect` - A vector to apply the map to
/// * `reduce_fn` - Accumulating function to apply to the list
/// * `initial` - An initial value to accumulate upon
///
/// # Example
///
/// ```
/// use sknife::collection::reduce;
/// let initial = 1;
/// let mut list = vec![1, 2, 3];
/// let fact = |acc, x| acc * x;
/// reduce(fact, list.as_mut_slice(), initial);
/// 
/// ```
/// 
/// # Result
/// ```
/// 6;
/// ```
pub fn reduce<F, A> (mut reduce_fn: F, vect: &mut [A], initial: A) -> A 
    where F: FnMut(A, A) -> A,
    A: Clone {
    let mut accum: A = initial;
    let mut iterator = vect.iter();

    loop {
        match iterator.next() {
            Some(v) => { 
                accum = reduce_fn(accum, v.clone()); 
            },
            None => { break; }
        }
    }

    accum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_fact() {
        let mut list = vec![1, 2, 3];
        let fact = |acc: i32, x| acc * x;
        assert_eq!(
            reduce(fact, list.as_mut_slice(), 1), 
            6
        );
    }

    #[test]
    fn reduce_sum() {
        let mut list = vec![1, 2, 3];
        let sum = |acc, x| acc + x;
        assert_eq!(
            reduce(sum, list.as_mut_slice(), 0), 
            6
        );
    }
}