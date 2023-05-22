

use std::borrow::Cow;

use regex::Regex;
use anyhow::Result;
use super::str_const::EMPTY_STR;

/// trait for different pattern match method
pub trait PatternMatch {
    /// if it contain the pattern
    fn contain(&self, line: &str) -> bool;
    /// extract the pattern
    fn extract(&self, line: &str) -> Option<String>;
    /// Replace the pattern
    fn replace(&self, line: &str, substitute: &str, times: usize) -> (bool, String);
}



/// match with regex
pub struct RegexMatcher {
    /// Regex itself 
    re: Regex,
    /// ignore case or not
    ignorecase: bool,
}


impl RegexMatcher {
    pub fn new(pattern: &str, ignorecase: bool) -> Result<RegexMatcher>{
        let new_pattern: String;
        if ignorecase {
            new_pattern = format!(r"(?i){}",pattern);
        } else {
            new_pattern = pattern.to_owned();
        }
        Ok (RegexMatcher {
            re: Regex::new(&new_pattern)?,
            ignorecase: ignorecase,
        })
    }
}

impl <'a> PatternMatch for RegexMatcher {
    /// if it contain the pattern
    fn contain(&self, line: &str) -> bool {
        self.re.is_match(line)
    }
    /// extract the pattern
    fn extract(&self, line: &str) -> Option<String> {
        let my_match = self.re.find(&line);
        match my_match {
            Some(v) => {
                return Some(v.as_str().to_owned());
            }
            None => return None,
        }  
    }
    /// Replace the pattern
    fn replace(&self, line: &str, substitute: &str, times: usize) -> (bool, String){
        let replaced_line =self.re.replacen(line, times, substitute);
        match replaced_line {
            Cow::Borrowed(v) => return ( false, line.to_owned()),
            Cow::Owned(v) => return ( true, v),
        }
    }
}

