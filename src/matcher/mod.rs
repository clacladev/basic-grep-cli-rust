use self::pattern::{parse_pattern, Pattern};
use std::{iter::Peekable, str::Chars};

mod pattern;
mod tests;

pub fn match_pattern(input_string: &str, pattern_string: &str) -> bool {
    let patterns = parse_pattern(pattern_string);
    is_matching(input_string, &patterns)
}

fn is_matching(input_string: &str, patterns: &[Pattern]) -> bool {
    let mut chars = input_string.chars().peekable();

    for pattern in patterns {
        let is_match = match pattern {
            Pattern::Literal(c) => is_matching_literal(c, &mut chars),
            Pattern::Digit => is_matching_digit(&mut chars),
            Pattern::Alphanumeric => is_matching_alphanumeric(&mut chars),
            Pattern::PositiveGroup(group) => is_matching_positive_group(group, &mut chars),
            Pattern::NegativeGroup(group) => is_matching_negative_group(group, &mut chars),
            Pattern::StartOfString(string) => is_matching_start_of_string(string, input_string),
            Pattern::EndOfString(string) => is_matching_end_of_string(string, input_string),
            Pattern::ZeroOrOne(c) => is_matching_zero_or_one(c, &mut chars),
            Pattern::OneOrMore(c) => is_matching_one_or_more(c, &mut chars),
            Pattern::Wildcard => is_matching_wildcard(&mut chars),
            Pattern::Alternation(groups) => is_matching_alternation(groups, &mut chars),
        };
        if !is_match {
            return false;
        }
    }

    true
}

fn is_matching_literal(c: &char, chars: &mut Peekable<Chars>) -> bool {
    while let Some(char) = chars.next() {
        if *c == char {
            return true;
        }
    }
    false
}

fn is_matching_digit(chars: &mut Peekable<Chars>) -> bool {
    while let Some(char) = chars.next() {
        if char.is_digit(10) {
            return true;
        }
    }
    false
}

fn is_matching_alphanumeric(chars: &mut Peekable<Chars>) -> bool {
    while let Some(char) = chars.next() {
        if char.is_alphanumeric() || char == '_' {
            return true;
        }
    }
    false
}

fn is_matching_positive_group(group: &String, chars: &mut Peekable<Chars>) -> bool {
    while let Some(char) = chars.next() {
        if group.contains(char) {
            return true;
        }
    }
    false
}

fn is_matching_negative_group(group: &String, chars: &mut Peekable<Chars>) -> bool {
    while let Some(char) = chars.next() {
        if group.contains(char) {
            return false;
        }
    }
    true
}

fn is_matching_start_of_string(string: &String, input_string: &str) -> bool {
    input_string.starts_with(string)
}

fn is_matching_end_of_string(string: &String, input_string: &str) -> bool {
    input_string.ends_with(string)
}

fn is_matching_zero_or_one(c: &char, chars: &mut Peekable<Chars>) -> bool {
    let mut count: usize = 0;
    while let Some(char) = chars.peek() {
        if *c != *char {
            break;
        }
        chars.next();
        count += 1;
    }
    count <= 1
}

fn is_matching_one_or_more(c: &char, chars: &mut Peekable<Chars>) -> bool {
    let mut count: usize = 0;
    while let Some(char) = chars.peek() {
        if *c != *char {
            break;
        }
        chars.next();
        count += 1;
    }
    count >= 1
}

fn is_matching_wildcard(chars: &mut Peekable<Chars>) -> bool {
    match chars.next() {
        Some(_) => true,
        None => false,
    }
}

fn is_matching_alternation(groups: &Vec<Vec<Pattern>>, chars: &mut Peekable<Chars>) -> bool {
    let remaining_string: String = chars.collect();
    for group in groups {
        if is_matching(&remaining_string, group) {
            return true;
        }
    }
    false
}
