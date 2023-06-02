use std::borrow::Cow;

use anyhow::Result;
use regex::Regex;

/// trait for different pattern match method
pub trait PatternMatch: Sync + Send {
    /// if it contain the pattern
    fn contain(&self, line: &str) -> bool;
    /// extract the pattern
    fn extract(&self, line: &str) -> Vec<(usize, usize)>;
    /// Replace the pattern
    fn replace<'a>(&self, line: &'a str, substitute: &'a str, times: usize) -> Cow<'a, str>;
}

/// match with regex
#[derive(Clone)]
pub struct RegexMatcher {
    /// Regex itself
    re: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str, ignorecase: bool) -> Result<RegexMatcher> {
        let new_pattern: String;
        if ignorecase {
            new_pattern = format!(r"(?i){}", pattern);
        } else {
            new_pattern = pattern.to_owned();
        }
        Ok(RegexMatcher {
            re: Regex::new(&new_pattern)?,
        })
    }
}

impl PatternMatch for RegexMatcher {
    /// if it contain the pattern
    fn contain(&self, line: &str) -> bool {
        self.re.is_match(line)
    }
    /// extract pos
    fn extract(&self, line: &str) -> Vec<(usize, usize)> {
        let mut match_pos_vec: Vec<(usize, usize)> = vec![];
        for capture_groups in self.re.captures_iter(line) {
            for my_match in capture_groups.iter() {
                match my_match {
                    Some(v) => {
                        match_pos_vec.push((v.start(), v.end()));
                    }
                    None => (),
                }
            }
        }
        match_pos_vec
    }
    /// Replace the pattern
    fn replace<'a>(&self, line: &'a str, substitute: &'a str, times: usize) -> Cow<'a, str> {
        self.re.replacen(line, times, substitute)
    }
}
