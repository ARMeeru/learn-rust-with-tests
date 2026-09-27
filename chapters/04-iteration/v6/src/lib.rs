// ANCHOR: code
/// Repeats a character the given number of times.
///
/// ```
/// use ch04_iteration_v6::repeat;
///
/// let repeated = repeat('a', 5);
/// assert_eq!(repeated, "aaaaa");
/// ```
pub fn repeat(character: char, count: usize) -> String {
    std::iter::repeat_n(character, count).collect()
}
// ANCHOR_END: code

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: test
    #[test]
    fn repeats_a_character_the_given_number_of_times() {
        let repeated = repeat('a', 8);
        let expected = "aaaaaaaa";
        assert_eq!(repeated, expected);
    }
    // ANCHOR_END: test
}
