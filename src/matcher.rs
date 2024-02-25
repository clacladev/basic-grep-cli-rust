use self::pattern::{parse_pattern, Pattern};

mod pattern;

pub fn match_pattern(input_string: &str, pattern_string: &str) -> bool {
    let input_string_chars = input_string.chars().collect::<Vec<char>>();
    let input_string_length = input_string_chars.len();
    let patterns = parse_pattern(pattern_string);
    let mut chars_check_count = 0;

    while input_string_length > chars_check_count {
        let remaining_string = &input_string_chars[chars_check_count..]
            .iter()
            .collect::<String>();
        let match_result = is_matching(remaining_string, &patterns);
        chars_check_count += match_result.chars_check_count;
        if !match_result.should_search_next_char {
            return match_result.is_match;
        }
    }

    false
}

struct MatchResult {
    is_match: bool,
    chars_check_count: usize,
    should_search_next_char: bool,
}

impl MatchResult {
    fn new(is_match: bool, chars_check_count: usize, should_search_next_char: bool) -> MatchResult {
        MatchResult {
            is_match,
            chars_check_count,
            should_search_next_char,
        }
    }
}

fn is_matching(input_string: &str, patterns: &[Pattern]) -> MatchResult {
    let mut chars = input_string.chars();
    let input_string_length = input_string.chars().count();

    'patterns_loop: for pattern in patterns {
        let char = match chars.next() {
            Some(char) => char,
            None => return MatchResult::new(false, 0, false),
        };
        let match_result = match pattern {
            Pattern::Literal(c) => {
                let mut char = char;
                loop {
                    if *c == char {
                        continue 'patterns_loop;
                    }
                    char = match chars.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(false, input_string_length - chars.clone().count(), false)
            }
            Pattern::Digit => {
                let mut char = char;
                loop {
                    if char.is_digit(10) {
                        continue 'patterns_loop;
                    }
                    char = match chars.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(false, input_string_length - chars.clone().count(), false)
            }
            Pattern::Alphanumeric => {
                let mut char = char;
                loop {
                    if char.is_alphanumeric() || char == '_' {
                        continue 'patterns_loop;
                    }
                    char = match chars.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(false, input_string_length - chars.clone().count(), false)
            }
            Pattern::PositiveGroup(group) => {
                let mut char = char;
                loop {
                    if group.contains(char) {
                        return MatchResult::new(
                            true,
                            input_string_length - chars.clone().count(),
                            false,
                        );
                    }
                    char = match chars.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(false, input_string_length - chars.clone().count(), false)
            }
            Pattern::NegativeGroup(group) => {
                let mut char = char;
                loop {
                    if group.contains(char) {
                        return MatchResult::new(
                            false,
                            input_string_length - chars.clone().count(),
                            false,
                        );
                    }
                    char = match chars.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(true, input_string_length - chars.clone().count(), false)
            }
            Pattern::StartOfString(string) => {
                let is_match = input_string.starts_with(string);
                if is_match {
                    chars.nth(string.len() - 1); // TODO: Maybe -2?
                    continue 'patterns_loop;
                }
                MatchResult::new(false, string.len(), false)
            }
            Pattern::EndOfString(string) => {
                let is_match = input_string.ends_with(string);
                if is_match {
                    chars.nth(string.len() - 1);
                }
                MatchResult::new(is_match, string.len(), false)
            }
            Pattern::OneOrMore(c) => {
                let mut chars_copy = chars.clone();
                let mut char = char;
                let mut count = 0;
                loop {
                    if *c != char {
                        break;
                    }
                    count += 1;
                    char = match chars_copy.next() {
                        Some(char) => char,
                        None => break,
                    };
                }
                MatchResult::new(count >= 1, count, false)
            }
        };

        if !match_result.is_match {
            return match_result;
        }
    }

    MatchResult::new(true, input_string_length - chars.count(), false)
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
        assert_eq!(match_pattern("hello world", "hel"), true);
        assert_eq!(match_pattern("hello world", "hwd"), true);
        assert_eq!(match_pattern("hello world", "hez"), false);
        assert_eq!(match_pattern("123e1z2h3", "hez"), false);
        assert_eq!(match_pattern("123h1e2z3", "hez"), true);
    }

    #[test]
    fn test_match_pattern_single_digits() {
        assert_eq!(match_pattern("hello world", r"\d"), false);
        assert_eq!(match_pattern("hello 1 world", r"\d"), true);
        assert_eq!(match_pattern("2 hello world", r"\d"), true);
        assert_eq!(match_pattern("hello world 3", r"\d"), true);
        assert_eq!(match_pattern("Cia0", r"\d"), true);
        assert_eq!(match_pattern("Cia0", r"\d"), true);
        assert_eq!(match_pattern("sally has 1 orange", r"\d orange"), true);
        assert_eq!(match_pattern("sally has 1 orange", r"\d apple"), false);
        assert_eq!(match_pattern("orange 2", r"orange \d"), true);
        assert_eq!(match_pattern("orange 2", r"apple \d"), false);
    }

    #[test]
    fn test_match_pattern_alphanumeric() {
        assert_eq!(match_pattern("hello", r"\w"), true);
        assert_eq!(match_pattern("2123", r"\w"), true);
        assert_eq!(match_pattern("___", r"\w"), true);
        assert_eq!(match_pattern("_he110_", r"\w"), true);
        assert_eq!(match_pattern("£$%a", r"\w"), true);
        assert_eq!(match_pattern("£$%A", r"\w"), true);
        assert_eq!(match_pattern("£$%6", r"\w"), true);
        assert_eq!(match_pattern("£$%_", r"\w"), true);
        assert_eq!(match_pattern("£$%", r"\w"), false);
        assert_eq!(match_pattern("---", r"\w"), false);
        assert_eq!(match_pattern("é", r"\w"), true);
        assert_eq!(match_pattern("ç", r"\w"), true);
        assert_eq!(match_pattern("#A#", r"\w\w"), false);
        assert_eq!(match_pattern("sally has 3 dogs", r"\d \w\w\ws"), true);
        assert_eq!(match_pattern("sally has 4 dogs", r"\d \w\w\ws"), true);
        assert_eq!(match_pattern("sally has 1 dog", r"\d \w\w\ws"), false);
        assert_eq!(match_pattern("a 1 dog", r"\d \w\w\ws"), false);
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
        assert_eq!(match_pattern("hello world", "[^abcd]"), false);
        assert_eq!(match_pattern("hello world", "[^etz]"), false);
        assert_eq!(match_pattern("hello world", "[^cd]"), false);
        assert_eq!(match_pattern("hello world", "[^abctyj]"), true);
        assert_eq!(match_pattern("hello world", "[^abctyjh]"), false);
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

    #[test]
    fn test_match_pattern_one_or_more() {
        assert_eq!(match_pattern("log", "log+"), true);
        assert_eq!(match_pattern("loggg", "log+"), true);
        assert_eq!(match_pattern("logs", "log+s"), true);
        assert_eq!(match_pattern("logggs", "log+s"), true);
        assert_eq!(match_pattern("logs", "a+"), false);
        assert_eq!(match_pattern("los", "log+"), false);
        assert_eq!(match_pattern("log", "a+og"), false);
    }
}
