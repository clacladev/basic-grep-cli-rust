#[derive(Debug, PartialEq)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
    StartOfString(String),
    EndOfString(String),
    OneOrMore(char),
    // ZeroOrOne(char),
}

pub fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern.chars().peekable();

    // End of string
    if pattern.ends_with('$') {
        chars.next_back();
        let remaining = chars.collect::<String>();
        patterns.push(Pattern::EndOfString(remaining));
        return patterns;
    }

    while let Some(char) = chars.next() {
        // Groups
        if char == '[' {
            let mut group = String::new();
            while let Some(char) = chars.next() {
                if char == ']' {
                    break;
                }
                group.push(char);
            }
            if group.starts_with('^') {
                patterns.push(Pattern::NegativeGroup(group[1..].to_string()));
            } else {
                patterns.push(Pattern::PositiveGroup(group));
            }
            continue;
        }

        // Start of string
        if char == '^' {
            let remaining = chars.clone().collect::<String>();
            patterns.push(Pattern::StartOfString(remaining));
            break;
        }

        // Escape sequences
        if char == '\\' {
            match chars.next() {
                Some('d') => patterns.push(Pattern::Digit),
                Some('w') => patterns.push(Pattern::Alphanumeric),
                Some(c) => patterns.push(Pattern::Literal(c)),
                None => panic!("Invalid escape sequence"),
            }
            continue;
        }

        // One or more
        if let Some('+') = chars.peek() {
            patterns.push(Pattern::OneOrMore(char));
            chars.next();
            continue;
        }

        // Literal
        patterns.push(Pattern::Literal(char));
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern_with_literal() {
        assert_eq!(parse_pattern("h"), vec![Pattern::Literal('h')]);
        assert_eq!(parse_pattern("z"), vec![Pattern::Literal('z')]);
        assert_eq!(
            parse_pattern("zoz"),
            vec![
                Pattern::Literal('z'),
                Pattern::Literal('o'),
                Pattern::Literal('z')
            ]
        );
    }

    #[test]
    fn test_parse_pattern_with_digit() {
        assert_eq!(parse_pattern(r"\d"), vec![Pattern::Digit]);
        assert_eq!(parse_pattern(r"\d\d"), vec![Pattern::Digit, Pattern::Digit]);
    }

    #[test]
    fn test_parse_pattern_with_alphanumeric() {
        assert_eq!(parse_pattern(r"\w"), vec![Pattern::Alphanumeric]);
        assert_eq!(
            parse_pattern(r"\w\w"),
            vec![Pattern::Alphanumeric, Pattern::Alphanumeric]
        );
    }

    #[test]
    fn test_parse_pattern_with_positive_group() {
        assert_eq!(
            parse_pattern("[a]"),
            vec![Pattern::PositiveGroup("a".to_string())]
        );
        assert_eq!(
            parse_pattern("[abc]"),
            vec![Pattern::PositiveGroup("abc".to_string())]
        );
    }

    #[test]
    fn test_parse_pattern_with_negative_group() {
        assert_eq!(
            parse_pattern("[^a]"),
            vec![Pattern::NegativeGroup("a".to_string())]
        );
        assert_eq!(
            parse_pattern("[^abc]"),
            vec![Pattern::NegativeGroup("abc".to_string())]
        );
    }

    #[test]
    fn test_parse_pattern_with_start_of_string() {
        assert_eq!(
            parse_pattern("^h"),
            vec![Pattern::StartOfString("h".to_string())]
        );
        assert_eq!(
            parse_pattern("^abc"),
            vec![Pattern::StartOfString("abc".to_string())]
        );
        assert_eq!(
            parse_pattern("^hey"),
            vec![Pattern::StartOfString("hey".to_string())]
        );
    }

    #[test]
    fn test_parse_pattern_with_end_of_string() {
        assert_eq!(
            parse_pattern("h$"),
            vec![Pattern::EndOfString("h".to_string())]
        );
        assert_eq!(
            parse_pattern("abc$"),
            vec![Pattern::EndOfString("abc".to_string())]
        );
        assert_eq!(
            parse_pattern("hey$"),
            vec![Pattern::EndOfString("hey".to_string())]
        );
    }

    #[test]
    fn test_parse_pattern_with_one_or_more() {
        assert_eq!(parse_pattern("h+"), vec![Pattern::OneOrMore('h')]);
        assert_eq!(parse_pattern("A+"), vec![Pattern::OneOrMore('A')]);
    }

    #[test]
    fn test_parse_pattern_with_combinations_of_patterns() {
        assert_eq!(
            parse_pattern("[a][b]"),
            vec![
                Pattern::PositiveGroup("a".to_string()),
                Pattern::PositiveGroup("b".to_string())
            ]
        );
        assert_eq!(
            parse_pattern("[a]b"),
            vec![
                Pattern::PositiveGroup("a".to_string()),
                Pattern::Literal('b')
            ]
        );
        assert_eq!(
            parse_pattern("a[bc]"),
            vec![
                Pattern::Literal('a'),
                Pattern::PositiveGroup("bc".to_string())
            ]
        );
        assert_eq!(
            parse_pattern("a[^bc]"),
            vec![
                Pattern::Literal('a'),
                Pattern::NegativeGroup("bc".to_string())
            ]
        );
        assert_eq!(
            parse_pattern(r"\d\d\ds"),
            vec![
                Pattern::Digit,
                Pattern::Digit,
                Pattern::Digit,
                Pattern::Literal('s'),
            ]
        );
        assert_eq!(
            parse_pattern(r"\d\w\dxxx"),
            vec![
                Pattern::Digit,
                Pattern::Alphanumeric,
                Pattern::Digit,
                Pattern::Literal('x'),
                Pattern::Literal('x'),
                Pattern::Literal('x'),
            ]
        );
        assert_eq!(
            parse_pattern(r"^yolo"),
            vec![Pattern::StartOfString("yolo".to_string())]
        );
        assert_eq!(
            parse_pattern("ab+c"),
            vec![
                Pattern::Literal('a'),
                Pattern::OneOrMore('b'),
                Pattern::Literal('c')
            ]
        );
        assert_eq!(
            parse_pattern("hey+"),
            vec![
                Pattern::Literal('h'),
                Pattern::Literal('e'),
                Pattern::OneOrMore('y')
            ]
        );
    }
}
