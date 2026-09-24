/// Takes two integers and returns the sum of them.
///
/// ```
/// use ch03_integers_v5::add;
///
/// let sum = add(1, 5);
/// assert_eq!(sum, 6);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        let sum = add(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }

    // ANCHOR: overflow_test
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn adding_past_the_maximum_panics() {
        add(i32::MAX, 1);
    }
    // ANCHOR_END: overflow_test
}
