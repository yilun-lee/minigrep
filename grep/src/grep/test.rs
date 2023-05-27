



#[cfg(test)]
mod tests {
    use crate::grep::matcher::{RegexMatcher, PatternMatch};
    use crate::grep::handler::{LineReplacer,ReplaceLine, LinePainter, LineExtractor};

    #[test]
    fn test_regex_matcher() {
        let my_str = "AABc____4567__AABC";
        let my_re = RegexMatcher::new(r"[A-Za-z]+", true).unwrap();
        let replacer = "I_AM_REPLACED";
        
        let out = my_re.contain(my_str);
        println!("contain: {}",out);

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

    #[test]
    fn test_painter() {
        let line = "ldsf_AA_rlkmg_AAglknlkg_AArmgerAAA";
        let my_re = RegexMatcher::new(r"(A+)", true).unwrap();
        
        let my_line_replacer = LinePainter {
            matcher: Box::new(my_re),
        };

        let (match_flag, match_line) = my_line_replacer.replace_line(line);
        //assert_eq!(match_flag, false);
        println!("{}",match_line);

    }


    #[test]
    fn test_extractor() {
        let line = "ldsf_AA_rlkmg_AAglknlkg_AArmgerAAA";
        let my_re = RegexMatcher::new(r"A+", true).unwrap();
        
        let my_line_replacer = LineExtractor {
            matcher: Box::new(my_re),
        };

        let (match_flag, match_line) = my_line_replacer.replace_line(line);
        //assert_eq!(match_flag, false);
        println!("{}",match_line);

    }

}

// cargo test -p grep test_loop  -- --show-output