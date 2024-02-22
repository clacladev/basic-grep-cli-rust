const DIGITS_PATTERN: &str = r"\d";
const ALPHANUMERIC_PATTERN: &str = r"\w";

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    }

    if pattern == DIGITS_PATTERN {
        return input_line.chars().any(|c| c.is_digit(10));
    }

    if pattern == ALPHANUMERIC_PATTERN {
        return input_line.chars().any(|c| c.is_alphanumeric() || c == '_');
    }

    if let Some(characters) = get_positive_character_group(pattern) {
        return input_line.chars().any(|c| characters.contains(&c));
    }

    panic!("Unhandled pattern: {}", pattern)
}

fn get_positive_character_group(pattern: &str) -> Option<Vec<char>> {
    if !pattern.starts_with('[') || !pattern.ends_with(']') {
        return None;
    }
    let characters = pattern.chars().skip(1).take(pattern.len() - 2).collect();
    Some(characters)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_match_pattern_positive_character_group() {
        assert_eq!(match_pattern("hello world", "[abc]"), false);
        assert_eq!(match_pattern("hello world", "[abcd]"), true);
        assert_eq!(match_pattern("hello world", "[etz]"), true);
        assert_eq!(match_pattern("hello world", "[cd]"), true);
        assert_eq!(match_pattern("hello world", "[abctyj]"), false);
        assert_eq!(match_pattern("hello world", "[abctyjh]"), true);
    }
}
