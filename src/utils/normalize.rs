use regex::Regex;

/**
* Normlizes the input string by converting it to lowercase and removing all non-alphanumeric characters.
* @param text: The input string to be normalized.
* @return: A normalized string with only lowercase alphanumeric characters and spaces.
*/
pub fn normalize(text: &str) -> String {
    let text = text.to_lowercase();
    let re = Regex::new(r"[^a-z0-9\s]").unwrap();
    let text = re.replace_all(&text, "");
    text.trim().to_string()
}
