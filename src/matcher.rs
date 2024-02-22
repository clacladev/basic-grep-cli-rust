const DIGITS_PATTERN: &str = r"\d";
const ALPHANUMERIC_PATTERN: &str = r"\w";

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == DIGITS_PATTERN {
        return input_line.chars().any(|c| c.is_digit(10));
    } else if pattern == ALPHANUMERIC_PATTERN {
        return input_line.chars().any(|c| c.is_alphanumeric() || c == '_');
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
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
}
