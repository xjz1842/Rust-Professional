/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n <= 0 {
        return 0;
    }
    let matrix = [[1, 1], [1, 0]];
    let power = n - 1;
    let result_matrix = matrix_pow(matrix, power);
    result_matrix[0][0]
}

fn multiply(m1: [[i32; 2]; 2], m2: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let a = m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0];
    let b = m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1];
    let c = m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0];
    let d = m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1];
    [[a, b], [c, d]]
}

fn matrix_pow(m: [[i32; 2]; 2], mut power: i32) -> [[i32; 2]; 2] {
    let mut result = [[1, 0], [0, 1]]; // Identity matrix
    let mut base = m;
    while power > 0 {
        if power % 2 == 1 {
            result = multiply(result, base);
        }
        base = multiply(base, base);
        power /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
