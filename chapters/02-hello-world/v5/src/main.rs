// ANCHOR: code
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";

fn hello(name: &str) -> String {
    format!("{ENGLISH_HELLO_PREFIX}{name}")
}
// ANCHOR_END: code

fn main() {
    println!("{}", hello("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_to_people() {
        let got = hello("Chris");
        let want = "Hello, Chris";
        assert_eq!(got, want);
    }
}
