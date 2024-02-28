#[cfg(test)]
mod tests {
    use crate::matcher::pattern::{parse_pattern, Pattern};

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
    fn test_parse_pattern_with_capturing_group() {
        assert_eq!(
            parse_pattern("(a)"),
            vec![Pattern::CapturingGroup(vec![Pattern::Literal('a')])]
        );
        assert_eq!(
            parse_pattern("(c\\d)"),
            vec![Pattern::CapturingGroup(vec![
                Pattern::Literal('c'),
                Pattern::Digit
            ])]
        );
    }

    #[test]
    fn test_parse_pattern_with_alternation() {
        assert_eq!(
            parse_pattern("(a|b)"),
            vec![Pattern::Alternation(vec![
                vec![Pattern::Literal('a')],
                vec![Pattern::Literal('b')]
            ])]
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
    fn test_parse_pattern_with_backreference() {
        assert_eq!(
            parse_pattern("(a) \\1.(b) \\2"),
            vec![
                Pattern::CapturingGroup(vec![Pattern::Literal('a')]),
                Pattern::Literal(' '),
                Pattern::Backreference(1),
                Pattern::Wildcard,
                Pattern::CapturingGroup(vec![Pattern::Literal('b')]),
                Pattern::Literal(' '),
                Pattern::Backreference(2)
            ]
        );
        assert_eq!(
            parse_pattern("(a|b).\\1.\\2"),
            vec![
                Pattern::Alternation(vec![
                    vec![Pattern::Literal('a')],
                    vec![Pattern::Literal('b')]
                ]),
                Pattern::Wildcard,
                Pattern::Backreference(1),
                Pattern::Wildcard,
                Pattern::Backreference(2)
            ]
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
            parse_pattern("(c?**\\w)"),
            vec![Pattern::CapturingGroup(vec![
                Pattern::ZeroOrOne('c'),
                Pattern::Literal('*'),
                Pattern::Literal('*'),
                Pattern::Alphanumeric
            ])]
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
        assert_eq!(
            parse_pattern("(.a)_\\1.(b+)_\\2"),
            vec![
                Pattern::CapturingGroup(vec![Pattern::Wildcard, Pattern::Literal('a')]),
                Pattern::Literal('_'),
                Pattern::Backreference(1),
                Pattern::Wildcard,
                Pattern::CapturingGroup(vec![Pattern::OneOrMore('b')]),
                Pattern::Literal('_'),
                Pattern::Backreference(2)
            ]
        );
        assert_eq!(
            parse_pattern("(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times"),
            vec![
                Pattern::CapturingGroup(vec![
                    Pattern::Alphanumeric,
                    Pattern::Alphanumeric,
                    Pattern::Alphanumeric,
                    Pattern::Alphanumeric,
                    Pattern::Literal(' '),
                    Pattern::Digit,
                    Pattern::Digit,
                    Pattern::Digit,
                ]),
                Pattern::Literal(' '),
                Pattern::Literal('i'),
                Pattern::Literal('s'),
                Pattern::Literal(' '),
                Pattern::Literal('d'),
                Pattern::Literal('o'),
                Pattern::Literal('i'),
                Pattern::Literal('n'),
                Pattern::Literal('g'),
                Pattern::Literal(' '),
                Pattern::Backreference(1),
                Pattern::Literal(' '),
                Pattern::Literal('t'),
                Pattern::Literal('i'),
                Pattern::Literal('m'),
                Pattern::Literal('e'),
                Pattern::Literal('s'),
            ]
        );
    }
}
