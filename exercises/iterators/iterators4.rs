// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE: xBINj

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    /*let mut v = Vec::new();
    for i in 2..=num {
        v.push(i);
    }

    let mut total = 1;
    for i in v.into_iter() {
        total *= i;
    }

    total*/

    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_8() {
        assert_eq!(40320, factorial(8));
    }
}
