// ANCHOR: code
pub fn add(x: i32, y: i32) -> i32 {
    x + y
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
}
