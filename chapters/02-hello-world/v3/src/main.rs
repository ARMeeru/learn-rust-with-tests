// The name is ignored on purpose: this is the red state the next step fixes.
#[allow(unused_variables)]
// ANCHOR: code
fn hello(name: &str) -> String {
    "Hello, world".to_string()
}
// ANCHOR_END: code

fn main() {
    println!("{}", hello("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // This step is red on purpose: the test drives the next change.
    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn hello_to_people() {
        let got = hello("Chris");
        let want = "Hello, Chris";
        assert_eq!(got, want);
    }
}
