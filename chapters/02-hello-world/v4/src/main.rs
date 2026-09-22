// ANCHOR: code
fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

fn main() {
    println!("{}", hello("world"));
}
// ANCHOR_END: code

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
}
// ANCHOR_END: test
