#[cfg(test)]
mod tests {

    use crate::matcher::match_pattern;

    #[test]
    fn test_match_pattern_single_letter() {
        assert_eq!(match_pattern("hello world", "h"), true);
        assert_eq!(match_pattern("hello world", "w"), false);
        assert_eq!(match_pattern("hello world", "hello"), true);
        assert_eq!(match_pattern("hello world", "ello"), false);
        assert_eq!(match_pattern("hello world", "hel"), true);
        assert_eq!(match_pattern("hello world", "hwd"), false);
        assert_eq!(match_pattern("hello world", "hez"), false);
        assert_eq!(match_pattern("123e1z2h3", "hez"), false);
        assert_eq!(match_pattern("123h1e2z3", "123"), true);
    }

    #[test]
    fn test_match_pattern_single_digits() {
        assert_eq!(match_pattern("world", r"\d"), false);
        assert_eq!(match_pattern("1 world", r"\d"), true);
        assert_eq!(match_pattern("2world", r"\d"), true);
        assert_eq!(match_pattern("h3llo", r"\d"), false);
        assert_eq!(match_pattern("h3llo", r"h\d"), true);
        assert_eq!(match_pattern("Cia0", r"\d"), false);
        assert_eq!(match_pattern("1 orange", r"\d orange"), true);
        assert_eq!(match_pattern("1 orange", r"\d apple"), false);
        assert_eq!(match_pattern("orange 2", r"orange \d"), true);
        assert_eq!(match_pattern("orange 2", r"apple \d"), false);
        assert_eq!(match_pattern("sally has 3 apples", r"\d apple"), true);
        assert_eq!(match_pattern("sally has two apples", r"\d apple"), false);
    }

    #[test]
    fn test_match_pattern_alphanumeric() {
        assert_eq!(match_pattern("hello", r"\w"), true);
        assert_eq!(match_pattern("2123", r"\w"), true);
        assert_eq!(match_pattern("___", r"\w"), true);
        assert_eq!(match_pattern("_he110_", r"\w"), true);
        assert_eq!(match_pattern("_he110_", r"\wZ"), false);
        assert_eq!(match_pattern("£$%a", r"\w"), true);
        assert_eq!(match_pattern("$A", r"\w"), true);
        assert_eq!(match_pattern("xA", r"x\w"), true);
        assert_eq!(match_pattern("---", r"\w"), false);
        assert_eq!(match_pattern("é", r"\w"), true);
        assert_eq!(match_pattern("ç", r"\w"), true);
        assert_eq!(match_pattern("#A#", r"\w\w#"), false);
        assert_eq!(match_pattern("#A#", r"#\w#"), true);
        assert_eq!(match_pattern("3 dogs", r"\d \w\w\ws"), true);
        assert_eq!(match_pattern("has 4 dogs", r"has \d \w\w\ws"), true);
        assert_eq!(match_pattern("has 1 dog", r"has \d \w\w\ws"), false);
        assert_eq!(match_pattern("a 1 dog", r"a \d \w\w\ws"), false);
        assert_eq!(match_pattern("a 1 dog", r"a \d \w\w\w"), true);
    }

    #[test]
    fn test_match_pattern_positive_group() {
        assert_eq!(match_pattern("c", "[abc]"), true);
        assert_eq!(match_pattern("d", "[abc]"), false);
        assert_eq!(match_pattern("ab", "[abc][abc]"), true);
        assert_eq!(match_pattern("ad", "[abc][abc]"), false);
        assert_eq!(match_pattern("hello", "[ytz]"), false);
        assert_eq!(match_pattern("hello", "[abctyjh]"), true);
        assert_eq!(match_pattern("eh", "[abctyjh]"), true);
    }

    #[test]
    fn test_match_pattern_negative_group() {
        assert_eq!(match_pattern("c", "[^abc]"), false);
        assert_eq!(match_pattern("d", "[^abc]"), true);
        assert_eq!(match_pattern("ab", "[^abc][^abc]"), false);
        assert_eq!(match_pattern("ad", "[^abc][^abc]"), false);
        assert_eq!(match_pattern("xy", "[^abc][^abc]"), true);
        assert_eq!(match_pattern("hello", "[^ytz]"), true);
        assert_eq!(match_pattern("he", "[^abctyjh]"), true);
        assert_eq!(match_pattern("hh", "[^abctyjh]"), false);
    }

    #[test]
    fn test_match_pattern_start_of_string() {
        assert_eq!(match_pattern("hello world", "^abc"), false);
        assert_eq!(match_pattern("abcde", "^abc"), true);
        assert_eq!(match_pattern("ade", "^abc"), false);
        assert_eq!(match_pattern("hello world", "^hello"), true);
        assert_eq!(match_pattern("hello world", "^Hello"), false);
        assert_eq!(match_pattern("hello world", "^world"), false);
        assert_eq!(match_pattern("log", "^log"), true);
        assert_eq!(match_pattern("slog", "^alog"), false);
        assert_eq!(match_pattern("slog", "^log"), false);
        assert_eq!(match_pattern("log", "^(log)"), true);
        assert_eq!(match_pattern("one log", "^(log)"), false);
        assert_eq!(match_pattern("log", "^(\\w+)"), true);
        assert_eq!(match_pattern("log", "^[abl]og"), true);
        assert_eq!(match_pattern("log", "^([gol]+)"), true);
        assert_eq!(match_pattern("log", "^([abc]+)"), false);
    }

    #[test]
    fn test_match_pattern_end_of_string() {
        assert_eq!(match_pattern("log", "log$"), true);
        assert_eq!(match_pattern("log two", "log$"), false);
        assert_eq!(match_pattern("slog", ".log$"), true);
        assert_eq!(match_pattern("logs", "log$"), false);
        assert_eq!(match_pattern("logs", "(log)s$"), true);
        assert_eq!(match_pattern("log", "([gol]+)$"), true);
        assert_eq!(match_pattern("logs", "([gol]+)$"), false);
        assert_eq!(match_pattern("logs", "([gol]+)s$"), true);
        assert_eq!(match_pattern("logs", "(\\w+)$"), true);
        assert_eq!(match_pattern("logs", "(\\w+) $"), false);
    }

    #[test]
    fn test_match_pattern_zero_or_one() {
        assert_eq!(match_pattern("log", "log?"), true);
        assert_eq!(match_pattern("loggg", "log?"), false);
        assert_eq!(match_pattern("logs", "logs?"), true);
        assert_eq!(match_pattern("logs", "log?s"), true);
        assert_eq!(match_pattern("logggs", "log?s"), false);
        assert_eq!(match_pattern("logs", "a?"), true);
        assert_eq!(match_pattern("los", "log?"), true);
        // assert_eq!(match_pattern("log", "a?og"), true);
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

    #[test]
    fn test_match_pattern_wildcard() {
        assert_eq!(match_pattern("log", "l.g"), true);
        assert_eq!(match_pattern("loggg", "log.."), true);
        assert_eq!(match_pattern("logs", "...."), true);
        assert_eq!(match_pattern("lo", "..."), false);
        assert_eq!(match_pattern("lo", "...."), false);
    }

    #[test]
    fn test_match_pattern_capturing_group() {
        assert_eq!(match_pattern("fish", "(dog)"), false);
        assert_eq!(match_pattern("dog", "(dog)"), true);
        assert_eq!(match_pattern("doggo", "(dog)"), true);
        assert_eq!(match_pattern("cat", "(cat)"), true);
        assert_eq!(match_pattern("fish", "(f..h)"), true);
        assert_eq!(match_pattern("fish", "(..s?\\w)"), true);
    }

    #[test]
    fn test_match_pattern_alternation() {
        assert_eq!(match_pattern("fish", "(dog|cat)"), false);
        assert_eq!(match_pattern("dog", "(dog|cat)"), true);
        assert_eq!(match_pattern("doggo", "(dog|cat)"), true);
        assert_eq!(match_pattern("cat", "(dog|cat)"), true);
        assert_eq!(match_pattern("fish", "(dog|cat|f..h)"), true);
        assert_eq!(match_pattern("fish", "(dog|..s?\\w)"), true);
    }

    #[test]
    fn test_match_pattern_backreference() {
        assert_eq!(match_pattern("fish", "\\1"), false);
        assert_eq!(match_pattern("fish fish", "(fish) \\1"), true);
        assert_eq!(match_pattern("f f", "(f) \\1"), true);
        assert_eq!(match_pattern("dog", "(dog|cat)"), true);
        assert_eq!(match_pattern("doggo", "(dog|cat)"), true);
        assert_eq!(match_pattern("cat", "(dog|cat)"), true);
        assert_eq!(match_pattern("fish", "(dog|cat|f..h)"), true);
        assert_eq!(match_pattern("fish", "(dog|..s?\\w)"), true);
        assert_eq!(match_pattern("cat and dog", "(cat) and \\1"), false);
        assert_eq!(
            match_pattern(
                "grep 101 is doing grep 101 times",
                "(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times"
            ),
            true
        );
        assert_eq!(
            match_pattern("abcd is abcd, not efg", "([abcd]+) is \\1, not [^xyz]+"),
            true
        );
        assert_eq!(match_pattern("abcd is abcd", "([abcd]+) is \\1"), true);
        // echo "$?! 101 is doing $?! 101 times" | ./your_grep.sh -E "(\w\w\w \d\d\d) is doing \1 times"
    }
}
