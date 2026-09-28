// ANCHOR: code
const REPEAT_COUNT: usize = 5;

pub fn repeat(character: char) -> String {
    let mut repeated = String::with_capacity(REPEAT_COUNT * character.len_utf8());
    for _ in 0..REPEAT_COUNT {
        repeated.push(character);
    }
    repeated
}
// ANCHOR_END: code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeats_a_character_five_times() {
        let repeated = repeat('a');
        let expected = "aaaaa";
        assert_eq!(repeated, expected);
    }
}
