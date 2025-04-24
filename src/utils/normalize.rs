use regex::Regex;
use unicode_normalization::UnicodeNormalization;

/**
* Normlizes the input string by converting it to lowercase and removing all non-alphanumeric characters.
* @param text: The input string to be normalized.
* @return: A normalized string with only lowercase alphanumeric characters and spaces.
*/
pub fn normalize(text: &str) -> String {
    let text = text.to_lowercase();

    let no_accents: String = text
        .nfd()
        .filter(|c| c.is_ascii() || c.is_alphanumeric() || c.is_whitespace())
        .collect();

    let re = Regex::new(r"[^a-z0-9\s]").unwrap();
    let cleaned = re.replace_all(&no_accents, "");

    cleaned.trim().to_string()
}
