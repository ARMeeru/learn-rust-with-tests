// The argument is ignored on purpose: this is the red state the next step fixes.
#[allow(unused_variables)]
// ANCHOR: code
pub fn repeat(character: char) -> String {
    String::new()
}
// ANCHOR_END: code

#[cfg(test)]
mod tests {
    use super::*;

    // This step is red on purpose: the test drives the next change.
    // ANCHOR: test
    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn repeats_a_character_five_times() {
        let repeated = repeat('a');
        let expected = "aaaaa";
        assert_eq!(repeated, expected);
    }
    // ANCHOR_END: test
}
