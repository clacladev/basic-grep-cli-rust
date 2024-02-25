use self::pattern::{parse_pattern, Pattern};

mod pattern;

pub fn match_pattern(input_string: &str, pattern_string: &str) -> bool {
    let input_string_chars = input_string.chars().collect::<Vec<char>>();
    let patterns = parse_pattern(pattern_string);

    // If start of string patter exists, check if the input string starts with it
    let start_string_pattern = patterns.iter().find_map(|p| match p {
        Pattern::StartOfString(string) => Some(string),
        _ => None,
    });
    if let Some(start_string_pattern) = start_string_pattern {
        return input_string.starts_with(start_string_pattern);
    }

    // If end of string patter exists, check if the input string ends with it
    let end_string_pattern = patterns.iter().find_map(|p| match p {
        Pattern::EndOfString(string) => Some(string),
        _ => None,
    });
    if let Some(end_string_pattern) = end_string_pattern {
        return input_string.ends_with(end_string_pattern);
    }

    for char_index in 0..input_string_chars.len() {
        let remaining_input_string = &input_string_chars[char_index..].iter().collect::<String>();
        if is_matching(remaining_input_string, &patterns) {
            return true;
        }
    }

    false
}

fn is_matching(input_string: &str, patterns: &[Pattern]) -> bool {
    let mut chars = input_string.chars();

    for pattern in patterns {
        let char = match chars.next() {
            Some(char) => char,
            None => return false,
        };
        let is_match = match pattern {
            Pattern::Literal(c) => *c == char,
            Pattern::Digit => char.is_digit(10),
            Pattern::Alphanumeric => char.is_alphanumeric() || char == '_',
            Pattern::PositiveGroup(group) => group.contains(char), // TODO: check all chars in input string
            Pattern::NegativeGroup(group) => !group.contains(char), // TODO: check all chars in input string
            Pattern::StartOfString(_) | Pattern::EndOfString(_) => continue,
        };

        if !is_match {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    const DIGITS_PATTERN: &str = r"\d";
    const ALPHANUMERIC_PATTERN: &str = r"\w";

    #[test]
    fn test_match_pattern_single_letter() {
        assert_eq!(match_pattern("hello world", "h"), true);
        assert_eq!(match_pattern("hello world", "w"), true);
        assert_eq!(match_pattern("hello world", "f"), false);
        assert_eq!(match_pattern("hello world", "a"), false);
    }

    #[test]
    fn test_match_pattern_single_digits() {
        assert_eq!(match_pattern("hello world", DIGITS_PATTERN), false);
        assert_eq!(match_pattern("hello 1 world", DIGITS_PATTERN), true);
        assert_eq!(match_pattern("2 hello world", DIGITS_PATTERN), true);
        assert_eq!(match_pattern("hello world 3", DIGITS_PATTERN), true);
        assert_eq!(match_pattern("Cia0", DIGITS_PATTERN), true);
    }

    #[test]
    fn test_match_pattern_alphanumeric() {
        assert_eq!(match_pattern("hello", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("2123", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("___", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("_he110_", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("£$%a", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("£$%A", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("£$%6", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("£$%_", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("£$%", ALPHANUMERIC_PATTERN), false);
        assert_eq!(match_pattern("---", ALPHANUMERIC_PATTERN), false);
        assert_eq!(match_pattern("é", ALPHANUMERIC_PATTERN), true);
        assert_eq!(match_pattern("ç", ALPHANUMERIC_PATTERN), true);
    }

    #[test]
    fn test_match_pattern_positive_group() {
        assert_eq!(match_pattern("hello world", "[abc]"), false);
        assert_eq!(match_pattern("hello world", "[abcd]"), true);
        assert_eq!(match_pattern("hello world", "[etz]"), true);
        assert_eq!(match_pattern("hello world", "[cd]"), true);
        assert_eq!(match_pattern("hello world", "[abctyj]"), false);
        assert_eq!(match_pattern("hello world", "[abctyjh]"), true);
    }

    #[test]
    fn test_match_pattern_negative_group() {
        assert_eq!(match_pattern("hello world", "[^abc]"), true);
        // assert_eq!(match_pattern("hello world", "[^abcd]"), false); // d exists!!!
        // assert_eq!(match_pattern("hello world", "[^etz]"), false);
        // assert_eq!(match_pattern("hello world", "[^cd]"), false);
        // assert_eq!(match_pattern("hello world", "[^abctyj]"), true);
        // assert_eq!(match_pattern("hello world", "[^abctyjh]"), false);
    }

    #[test]
    fn test_match_pattern_start_of_string() {
        assert_eq!(match_pattern("hello world", "^abc"), false);
        assert_eq!(match_pattern("abcde", "^abc"), true);
        assert_eq!(match_pattern("hello world", "^hello"), true);
        assert_eq!(match_pattern("hello world", "^Hello"), false);
        assert_eq!(match_pattern("hello world", "^world"), false);
        assert_eq!(match_pattern("log", "^log"), true);
        assert_eq!(match_pattern("slog", "^log"), false);
    }

    #[test]
    fn test_match_pattern_end_of_string() {
        assert_eq!(match_pattern("log", "log$"), true);
        assert_eq!(match_pattern("slog", "log$"), true);
        assert_eq!(match_pattern("logs", "log$"), false);
    }
}
