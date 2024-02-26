const ESCAPE_SYMBOL: char = '\\';
const DIGIT_SYMBOL: char = 'd';
const ALPHANUMERIC_SYMBOL: char = 'w';
const START_OF_STRING_SYMBOL: char = '^';
const END_OF_STRING_SYMBOL: char = '$';
const ZERO_OR_ONE_SYMBOL: char = '?';
const ONE_OR_MORE_SYMBOL: char = '+';
const GROUP_START_SYMBOL: char = '[';
const GROUP_END_SYMBOL: char = ']';
const NEGATIVE_GROUP_SYMBOL: char = '^';
const WILDCARD_SYMBOL: char = '.';
const ALTERNATION_START_SYMBOL: char = '(';
const ALTERNATION_END_SYMBOL: char = ')';
const ALTERNATION_SEPARATOR_SYMBOL: char = '|';

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
    StartOfString(String),
    EndOfString(String),
    ZeroOrOne(char),
    OneOrMore(char),
    Wildcard,
    Alternation(Vec<Vec<Self>>),
}

pub fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern.chars().peekable();

    // End of string
    if pattern.ends_with(END_OF_STRING_SYMBOL) {
        chars.next_back();
        let remaining = chars.collect::<String>();
        patterns.push(Pattern::EndOfString(remaining));
        return patterns;
    }

    while let Some(char) = chars.next() {
        // Groups
        if char == GROUP_START_SYMBOL {
            let mut group = String::new();
            while let Some(char) = chars.next() {
                if char == GROUP_END_SYMBOL {
                    break;
                }
                group.push(char);
            }
            if group.starts_with(NEGATIVE_GROUP_SYMBOL) {
                patterns.push(Pattern::NegativeGroup(group[1..].to_string()));
            } else {
                patterns.push(Pattern::PositiveGroup(group));
            }
            continue;
        }

        // Start of string
        if char == START_OF_STRING_SYMBOL {
            let remaining = chars.clone().collect::<String>();
            patterns.push(Pattern::StartOfString(remaining));
            break;
        }

        // Escape sequences
        if char == ESCAPE_SYMBOL {
            match chars.next() {
                Some(DIGIT_SYMBOL) => patterns.push(Pattern::Digit),
                Some(ALPHANUMERIC_SYMBOL) => patterns.push(Pattern::Alphanumeric),
                Some(c) => patterns.push(Pattern::Literal(c)),
                None => panic!("Invalid escape sequence"),
            }
            continue;
        }

        // Alternation group
        if char == ALTERNATION_START_SYMBOL {
            let mut alternation_string = String::new();
            while let Some(char) = chars.next() {
                if char == ALTERNATION_END_SYMBOL {
                    break;
                }
                alternation_string.push(char);
            }
            let patterns_groups = alternation_string
                .split(ALTERNATION_SEPARATOR_SYMBOL)
                .map(parse_pattern)
                .collect();
            patterns.push(Pattern::Alternation(patterns_groups));
            continue;
        }

        // Zero or one
        if let Some(&ZERO_OR_ONE_SYMBOL) = chars.peek() {
            patterns.push(Pattern::ZeroOrOne(char));
            chars.next();
            continue;
        }

        // One or more
        if let Some(&ONE_OR_MORE_SYMBOL) = chars.peek() {
            patterns.push(Pattern::OneOrMore(char));
            chars.next();
            continue;
        }

        // Wildcard
        if char == WILDCARD_SYMBOL {
            patterns.push(Pattern::Wildcard);
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
    fn test_parse_pattern_with_zero_or_one() {
        assert_eq!(parse_pattern("h?"), vec![Pattern::ZeroOrOne('h')]);
        assert_eq!(parse_pattern("A?"), vec![Pattern::ZeroOrOne('A')]);
    }

    #[test]
    fn test_parse_pattern_with_one_or_more() {
        assert_eq!(parse_pattern("h+"), vec![Pattern::OneOrMore('h')]);
        assert_eq!(parse_pattern("A+"), vec![Pattern::OneOrMore('A')]);
    }

    #[test]
    fn test_parse_pattern_with_wildcard() {
        assert_eq!(parse_pattern("."), vec![Pattern::Wildcard]);
        assert_eq!(
            parse_pattern("d.g.o"),
            vec![
                Pattern::Literal('d'),
                Pattern::Wildcard,
                Pattern::Literal('g'),
                Pattern::Wildcard,
                Pattern::Literal('o')
            ]
        );
    }

    #[test]
    fn test_parse_pattern_with_alternation() {
        assert_eq!(
            parse_pattern("(a)"),
            vec![Pattern::Alternation(vec![vec![Pattern::Literal('a')]])]
        );
        assert_eq!(
            parse_pattern("(a|b|cc)"),
            vec![Pattern::Alternation(vec![
                vec![Pattern::Literal('a')],
                vec![Pattern::Literal('b')],
                vec![Pattern::Literal('c'), Pattern::Literal('c')]
            ])]
        );
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
            parse_pattern("ab?c"),
            vec![
                Pattern::Literal('a'),
                Pattern::ZeroOrOne('b'),
                Pattern::Literal('c')
            ]
        );
        assert_eq!(
            parse_pattern("hey?"),
            vec![
                Pattern::Literal('h'),
                Pattern::Literal('e'),
                Pattern::ZeroOrOne('y')
            ]
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
        assert_eq!(
            parse_pattern("h?e.y+"),
            vec![
                Pattern::ZeroOrOne('h'),
                Pattern::Literal('e'),
                Pattern::Wildcard,
                Pattern::OneOrMore('y')
            ]
        );
        assert_eq!(
            parse_pattern("(dog|.ss|f?i+)"),
            vec![Pattern::Alternation(vec![
                vec![
                    Pattern::Literal('d'),
                    Pattern::Literal('o'),
                    Pattern::Literal('g')
                ],
                vec![
                    Pattern::Wildcard,
                    Pattern::Literal('s'),
                    Pattern::Literal('s')
                ],
                vec![Pattern::ZeroOrOne('f'), Pattern::OneOrMore('i')]
            ])]
        );
    }
}
