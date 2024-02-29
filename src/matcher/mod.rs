use self::pattern::{parse_pattern, Pattern};
use std::{iter::Peekable, str::Chars};

mod pattern;
mod tests;

pub fn match_pattern(input_string: &str, pattern_string: &str) -> bool {
    let patterns = parse_pattern(pattern_string);
    let mut chars = input_string.chars().peekable();
    let (is_match, _) = is_matching(&patterns, &mut chars, false);
    is_match
}

fn is_matching(
    patterns: &[Pattern],
    mut chars: &mut Peekable<Chars>,
    from_start: bool,
) -> (bool, usize) {
    let initial_chars_count = chars.clone().count();
    for pattern in patterns {
        loop {
            let is_match = match pattern {
                Pattern::Literal(c) => is_matching_literal(c, &mut chars),
                Pattern::Digit => is_matching_digit(&mut chars),
                Pattern::Alphanumeric => is_matching_alphanumeric(&mut chars),
                Pattern::PositiveGroup(group) => is_matching_positive_group(group, &mut chars),
                Pattern::NegativeGroup(group) => is_matching_negative_group(group, &mut chars),
                Pattern::StartOfString(pattern) => {
                    let is_match = is_matching_start_of_string(pattern, &mut chars);
                    if !is_match {
                        return (false, initial_chars_count - chars.count());
                    }
                    true
                }
                Pattern::EndOfString => is_matching_end_of_string(&mut chars),
                Pattern::ZeroOrOne(pattern) => is_matching_zero_or_one(pattern, &mut chars),
                Pattern::OneOrMore(pattern) => is_matching_one_or_more(pattern, &mut chars),
                Pattern::Wildcard => is_matching_wildcard(&mut chars),
                Pattern::CapturingGroup(group) => is_matching_capturing_group(group, &mut chars),
                Pattern::Alternation(groups) => is_matching_alternation(groups, &mut chars),
                Pattern::Backreference(number) => {
                    is_matching_backreference(*number, &mut chars, patterns)
                }
            };
            // If it's a match, then continue with the next pattern
            if is_match {
                break;
            }
            // If it's not a match and it's the first pattern, then try with the next character if allowed
            let is_first_pattern = patterns.first() == Some(pattern);
            if is_first_pattern && !from_start {
                if let Some(_) = chars.peek() {
                    continue;
                }
            }
            // If it's not a match, if it's not the first pattern or if it cannot check the next char, then stop
            return (false, initial_chars_count - chars.count());
        }
    }

    (true, initial_chars_count - chars.count())
}

fn is_matching_literal(c: &char, chars: &mut Peekable<Chars>) -> bool {
    if let Some(char) = chars.next() {
        return *c == char;
    }
    false
}

fn is_matching_digit(chars: &mut Peekable<Chars>) -> bool {
    if let Some(char) = chars.next() {
        return char.is_digit(10);
    }
    false
}

fn is_matching_alphanumeric(chars: &mut Peekable<Chars>) -> bool {
    if let Some(char) = chars.next() {
        return char.is_alphanumeric() || char == '_';
    }
    false
}

fn is_matching_positive_group(group: &String, chars: &mut Peekable<Chars>) -> bool {
    if let Some(char) = chars.next() {
        return group.contains(char);
    }
    false
}

fn is_matching_negative_group(group: &String, chars: &mut Peekable<Chars>) -> bool {
    if let Some(char) = chars.next() {
        return !group.contains(char);
    }
    false
}

fn is_matching_start_of_string(pattern: &Pattern, chars: &mut Peekable<Chars>) -> bool {
    let (is_match, checked_chars_count) = is_matching(&[pattern.clone()], &mut chars.clone(), true);
    if is_match {
        chars.nth(checked_chars_count - 1);
    }
    is_match
}

fn is_matching_end_of_string(chars: &mut Peekable<Chars>) -> bool {
    match chars.next() {
        Some(_) => false,
        None => true,
    }
}

fn is_matching_zero_or_one(pattern: &Pattern, chars: &mut Peekable<Chars>) -> bool {
    let mut count: usize = 0;
    loop {
        let (is_match, checked_chars_count) =
            is_matching(&[pattern.clone()], &mut chars.clone(), true);
        if is_match {
            match chars.peek() {
                Some(_) => {
                    chars.nth(checked_chars_count - 1);
                }
                None => {}
            };
            count += 1;
            continue;
        }
        return count <= 1;
    }
}

fn is_matching_one_or_more(pattern: &Pattern, chars: &mut Peekable<Chars>) -> bool {
    let mut count: usize = 0;
    loop {
        let (is_match, checked_chars_count) =
            is_matching(&[pattern.clone()], &mut chars.clone(), true);
        if is_match {
            chars.nth(checked_chars_count - 1);
            count += 1;
            continue;
        }
        return count >= 1;
    }
}

fn is_matching_wildcard(chars: &mut Peekable<Chars>) -> bool {
    match chars.next() {
        Some(_) => true,
        None => false,
    }
}

fn is_matching_capturing_group(group: &Vec<Pattern>, chars: &mut Peekable<Chars>) -> bool {
    let (is_match, checked_chars_count) = is_matching(group, &mut chars.clone(), true);
    if is_match {
        chars.nth(checked_chars_count - 1);
    }
    is_match
}

fn is_matching_alternation(groups: &Vec<Vec<Pattern>>, chars: &mut Peekable<Chars>) -> bool {
    for group in groups {
        let (is_match, checked_chars_count) = is_matching(group, &mut chars.clone(), true);
        if is_match {
            chars.nth(checked_chars_count - 1);
            return true;
        }
    }
    false
}

fn is_matching_backreference(
    number: usize,
    chars: &mut Peekable<Chars>,
    patterns: &[Pattern],
) -> bool {
    let index = number - 1;
    let pattern = patterns
        .iter()
        .filter(|p| match p {
            Pattern::CapturingGroup(_) | Pattern::Alternation(_) => true,
            _ => false,
        })
        .nth(index);

    if let Some(pattern) = pattern {
        let (is_match, checked_chars_count) =
            is_matching(&[pattern.clone()], &mut chars.clone(), true);
        if is_match {
            chars.nth(checked_chars_count - 1);
            return true;
        }
    }

    false
}
