use std::vec::Vec;

pub fn fibonacci_series(n: u32) -> Vec<u32> {
    let mut result = vec![0, 1];

    if n == 0 {
        return vec![];
    } else if n == 1 {
        return vec![0];
    } else if n == 2 {
        return result;
    }

    for i in 2..n {
        let next = result[(i as usize) - 1] + result[(i as usize) - 2];
        result.push(next);
    }

    result
}
