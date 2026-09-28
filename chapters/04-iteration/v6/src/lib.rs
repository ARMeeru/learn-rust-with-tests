// ANCHOR: code
/// Repeats a character the given number of times.
///
/// ```
/// use ch04_iteration_v6::repeat;
///
/// let repeated = repeat('a', 5);
/// assert_eq!(repeated, "aaaaa");
/// ```
// ANCHOR_END: code
// ANCHOR: fix
pub fn repeat(character: char, count: usize) -> String {
    let mut repeated = String::with_capacity(count * character.len_utf8());
    for _ in 0..count {
        repeated.push(character);
    }
    repeated
}
// ANCHOR_END: fix

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
