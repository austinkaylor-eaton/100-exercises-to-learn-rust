// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_v: &'static mut [i32] = v.leak();
    let middle = static_v.len() / 2;
    let static_v_in_halves = static_v.split_at(middle);
    let left_side = static_v_in_halves.0;
    let right_side = static_v_in_halves.1;
    
    let left_thread = thread::spawn(move || {
        left_side.iter().sum::<i32>()
    });
    
    let right_thread = thread::spawn(move || {
        right_side.iter().sum::<i32>()
    });
    
    left_thread.join().unwrap() + right_thread.join().unwrap()
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
