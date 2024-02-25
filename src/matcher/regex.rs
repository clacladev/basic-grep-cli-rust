#[derive(Debug, PartialEq)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
}

pub fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern.chars();

    while let Some(char) = chars.next() {
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

        if char == '\\' {
            let char = chars.next();
            match char {
                Some('d') => patterns.push(Pattern::Digit),
                Some('w') => patterns.push(Pattern::Alphanumeric),
                Some(char) => patterns.push(Pattern::Literal(char)),
                None => panic!("Invalid escape sequence"),
            }
            continue;
        }

        patterns.push(Pattern::Literal(char));
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern() {
        assert_eq!(parse_pattern("h"), vec![Pattern::Literal('h')]);
        assert_eq!(parse_pattern(r"\d"), vec![Pattern::Digit]);
        assert_eq!(parse_pattern(r"\w"), vec![Pattern::Alphanumeric]);
        assert_eq!(
            parse_pattern("[^a]"),
            vec![Pattern::NegativeGroup("a".to_string())]
        );
        assert_eq!(
            parse_pattern("[a]"),
            vec![Pattern::PositiveGroup("a".to_string())]
        );
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
            parse_pattern("a[b]"),
            vec![
                Pattern::Literal('a'),
                Pattern::PositiveGroup("b".to_string())
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
    }
}
