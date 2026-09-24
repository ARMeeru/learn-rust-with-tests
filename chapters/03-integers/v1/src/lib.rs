// The arguments are ignored on purpose: this is the red state the next step fixes.
#[allow(unused_variables)]
// ANCHOR: code
pub fn add(x: i32, y: i32) -> i32 {
    0
}
// ANCHOR_END: code

#[cfg(test)]
mod tests {
    use super::*;

    // This step is red on purpose: the test drives the next change.
    // ANCHOR: test
    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn adds_two_numbers() {
        let sum = add(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }
    // ANCHOR_END: test
}
