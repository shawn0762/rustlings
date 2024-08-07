fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // let n = (1..num).take(5);

    // (2..=num).reduce(|f, n| f * n).unwrap_or(1)
    // (2..=num).fold(1, |acc, n| acc * n)
    (2..=num).product()
}

fn main() {
    // You can optionally experiment here.
    // let n = (1..4 + 1).reduce(|f, n| f * n).unwrap_or(1);
    // println!("{:?}", n);

    // let a = 1..=6;

    // (1..4).for_each(|n| println!("{:?}", n));
    // (1..=4).for_each(|n| println!("{:?}", n));

    // println!("{:?}", (2..=1).product::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
