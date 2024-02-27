mod tests;

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
