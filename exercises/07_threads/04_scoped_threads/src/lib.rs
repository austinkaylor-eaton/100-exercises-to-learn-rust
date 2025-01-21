// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let middle = v.len() / 2;
    
    let sum = thread::scope(|scope| {
        let left_thread = scope.spawn(|| {
            let left_side = &v[..middle];
            left_side.iter().sum::<i32>()
        });
        let right_thread = scope.spawn(|| {
            let right_side = &v[middle..];
            right_side.iter().sum::<i32>()
        });
        left_thread.join().unwrap() + right_thread.join().unwrap()
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
