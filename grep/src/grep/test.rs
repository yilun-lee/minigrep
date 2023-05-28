#[cfg(test)]
mod tests {
    use crate::grep::matcher::{PatternMatch, RegexMatcher};

    #[test]
    fn test_regex_matcher() {
        let my_str = "AABc____4567__AABC";
        let my_re = RegexMatcher::new(r"[A-Za-z]+", true).unwrap();

        let out = my_re.contain(my_str);
        println!("contain: {}", out);
    }
}

// cargo test -p grep test_loop  -- --show-output
