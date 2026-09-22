// ANCHOR: code
fn hello() -> &'static str {
    "Hello, world"
}

fn main() {
    println!("{}", hello());
}
// ANCHOR_END: code

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let got = hello();
        let want = "Hello, world";
        assert_eq!(got, want);
    }
}
// ANCHOR_END: test
