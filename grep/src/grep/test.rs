



#[cfg(test)]
mod tests {
    use crate::grep::matcher::{RegexMatcher, PatternMatch};
    use crate::grep::handler::{LineReplacer,ReplaceLine};

    #[test]
    fn test_regex_matcher() {
        let my_str = "AABc____4567__AABC";
        let my_re = RegexMatcher::new(r"AA.+", true).unwrap();
        let replacer = "I_AM_REPLACED";
        
        let out = my_re.contain(my_str);
        println!("contain: {}",out);

        let out = my_re.extract(my_str).ok_or(0).unwrap();
        println!("extract: {}",out);

        let (_, out) = my_re.replace(my_str, replacer, 0);
        println!("replace: {}",out);

    }

    #[test]
    fn test_replacer() {
        let line = "AABc____4567";
        let my_re = RegexMatcher::new(r"([ABC]+)_+(4567)", true).unwrap();
        
        let my_line_replacer = LineReplacer {
            matcher: Box::new(my_re),
            substitute: "I_AM_REPLACED",
            times: 0,
        };

        let (match_flag, match_line) = my_line_replacer.replace_line(line);
        assert_eq!(match_flag, true);
        println!("{}",match_line);
    }

}

// cargo test -p grep test_loop  -- --show-output