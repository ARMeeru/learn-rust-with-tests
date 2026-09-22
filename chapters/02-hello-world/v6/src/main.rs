// ANCHOR: code
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";

fn hello(name: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{ENGLISH_HELLO_PREFIX}{name}")
}
// ANCHOR_END: code

fn main() {
    println!("{}", hello(""));
}

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_to_people() {
        let got = hello("Chris");
        let want = "Hello, Chris";
        assert_eq!(got, want);
    }

    #[test]
    fn empty_string_defaults_to_world() {
        let got = hello("");
        let want = "Hello, World";
        assert_eq!(got, want);
    }
}
// ANCHOR_END: test
