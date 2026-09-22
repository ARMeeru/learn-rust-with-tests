// ANCHOR: hello
pub fn hello() -> &'static str {
    "Hello, world"
}
// ANCHOR_END: hello

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says_hello_world() {
        let got = hello();
        let want = "Hello, world";
        assert_eq!(got, want);
    }
}
// ANCHOR_END: test
