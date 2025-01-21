// TODO: Define a function named `squared` that raises all `i32`s 
//  within a slice to the power of 2.
//  The slice should be modified in place.

use std::slice::SliceIndex;

fn squared(mut slice: &mut [i32]) -> &[i32] {
    for s in slice.iter_mut() {
       *s *= *s;
    }
    slice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
