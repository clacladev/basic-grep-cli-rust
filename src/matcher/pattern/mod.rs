mod tests;

const ESCAPE_SYMBOL: char = '\\';
const DIGIT_SYMBOL: char = 'd';
const ALPHANUMERIC_SYMBOL: char = 'w';
const START_OF_STRING_SYMBOL: char = '^';
const END_OF_STRING_SYMBOL: char = '$';
const ZERO_OR_ONE_SYMBOL: char = '?';
const ONE_OR_MORE_SYMBOL: char = '+';
const POSITIVE_NEGATIVE_GROUP_START_SYMBOL: char = '[';
const POSITIVE_NEGATIVE_GROUP_END_SYMBOL: char = ']';
const NEGATIVE_GROUP_SYMBOL: char = '^';
const WILDCARD_SYMBOL: char = '.';
const CAPTURING_GROUP_START_SYMBOL: char = '(';
const CAPTURING_GROUP_END_SYMBOL: char = ')';
const ALTERNATION_SEPARATOR_SYMBOL: char = '|';

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
    StartOfString(Box<Self>),
    EndOfString,
    ZeroOrOne(Box<Self>),
    OneOrMore(Box<Self>),
    Wildcard,
    CapturingGroup(Vec<Self>),
    Alternation(Vec<Vec<Self>>),
    Backreference(usize),
}

pub fn parse_pattern(pattern_string: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern_string.chars().peekable();

    while let Some(char) = chars.next() {
        // Groups
        if char == POSITIVE_NEGATIVE_GROUP_START_SYMBOL {
            let mut group = String::new();
            while let Some(char) = chars.next() {
                if char == POSITIVE_NEGATIVE_GROUP_END_SYMBOL {
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
            let remaining_pattern_string = chars.clone().collect::<String>();
            let following_patterns = parse_pattern(&remaining_pattern_string);
            let Some(first_pattern) = following_patterns.first() else {
                panic!("Invalid pattern");
            };
            patterns.push(Pattern::StartOfString(Box::new(first_pattern.clone())));
            patterns.append(&mut following_patterns[1..].to_vec());
            return patterns;
        }

        // End of string
        if char == END_OF_STRING_SYMBOL {
            patterns.push(Pattern::EndOfString);
            if chars.next() != None {
                panic!("End of string pattern in the wrong position");
            }
            break;
        }

        // Escape sequences
        if char == ESCAPE_SYMBOL {
            match chars.next() {
                Some(DIGIT_SYMBOL) => patterns.push(Pattern::Digit),
                Some(ALPHANUMERIC_SYMBOL) => patterns.push(Pattern::Alphanumeric),
                Some(c) if c != '0' && c.is_digit(10) => patterns.push(Pattern::Backreference(
                    c.to_string().parse::<usize>().unwrap(),
                )),
                Some(c) => patterns.push(Pattern::Literal(c)),
                None => panic!("Invalid escape sequence"),
            }
            continue;
        }

        // Capturing or alternation group
        if char == CAPTURING_GROUP_START_SYMBOL {
            let mut alternation_string = String::new();
            while let Some(char) = chars.next() {
                if char == CAPTURING_GROUP_END_SYMBOL {
                    break;
                }
                alternation_string.push(char);
            }
            let patterns_groups: Vec<Vec<Pattern>> = alternation_string
                .split(ALTERNATION_SEPARATOR_SYMBOL)
                .map(parse_pattern)
                .collect();
            if patterns_groups.len() == 1 {
                patterns.push(Pattern::CapturingGroup(patterns_groups[0].clone()));
            } else {
                patterns.push(Pattern::Alternation(patterns_groups));
            }
            continue;
        }

        // Zero or one
        if char == ZERO_OR_ONE_SYMBOL {
            let Some(previous_pattern) = patterns.pop() else {
                panic!("Invalid pattern");
            };
            patterns.push(Pattern::ZeroOrOne(Box::new(previous_pattern)));
            continue;
        }

        // One or more
        if char == ONE_OR_MORE_SYMBOL {
            let Some(previous_pattern) = patterns.pop() else {
                panic!("Invalid pattern");
            };
            patterns.push(Pattern::OneOrMore(Box::new(previous_pattern)));
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
