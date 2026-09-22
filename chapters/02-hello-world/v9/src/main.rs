// ANCHOR: code
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";
const SPANISH_HELLO_PREFIX: &str = "Hola, ";
const FRENCH_HELLO_PREFIX: &str = "Bonjour, ";

enum Language {
    English,
    Spanish,
    French,
}

fn hello(name: &str, language: Language) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("{}{name}", greeting_prefix(language))
}

fn greeting_prefix(language: Language) -> &'static str {
    match language {
        Language::English => ENGLISH_HELLO_PREFIX,
        Language::Spanish => SPANISH_HELLO_PREFIX,
        Language::French => FRENCH_HELLO_PREFIX,
    }
}
// ANCHOR_END: code

fn main() {
    for language in [Language::English, Language::Spanish, Language::French] {
        println!("{}", hello("world", language));
    }
}

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_to_people() {
        let got = hello("Chris", Language::English);
        let want = "Hello, Chris";
        assert_eq!(got, want);
    }

    #[test]
    fn empty_string_defaults_to_world() {
        let got = hello("", Language::English);
        let want = "Hello, World";
        assert_eq!(got, want);
    }

    #[test]
    fn in_spanish() {
        let got = hello("Elodie", Language::Spanish);
        let want = "Hola, Elodie";
        assert_eq!(got, want);
    }

    #[test]
    fn in_french() {
        let got = hello("Lauren", Language::French);
        let want = "Bonjour, Lauren";
        assert_eq!(got, want);
    }
}
// ANCHOR_END: test
