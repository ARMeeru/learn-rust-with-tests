/// Takes two integers and returns the sum of them.
///
/// ```
/// use ch03_integers_v6::add;
///
/// let sum = add(1, 5);
/// assert_eq!(sum, 6);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

// ANCHOR: code
/// Adds two integers, or returns `None` if the sum does not fit in an `i32`.
///
/// ```
/// use ch03_integers_v6::try_add;
///
/// assert_eq!(try_add(1, 5), Some(6));
/// assert_eq!(try_add(i32::MAX, 1), None);
/// ```
pub fn try_add(x: i32, y: i32) -> Option<i32> {
    x.checked_add(y)
}
// ANCHOR_END: code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        let sum = add(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn adding_past_the_maximum_panics() {
        add(i32::MAX, 1);
    }

    // ANCHOR: try_add_tests
    #[test]
    fn try_add_adds_two_numbers() {
        assert_eq!(try_add(2, 2), Some(4));
    }

    #[test]
    fn try_add_reports_overflow() {
        assert_eq!(try_add(i32::MAX, 1), None);
    }
    // ANCHOR_END: try_add_tests
}
