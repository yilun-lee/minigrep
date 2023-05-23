
use super::matcher::RegexMatcher;
use super::matcher::PatternMatch;
/// trait for match a line and return bool, for match only
/// May be more efficinet
/// not used for now
pub trait MatchLine <'a>{
    fn match_line(&self, line: &'a str,) -> bool;
}

/// trait for match a line and return bool and line
/// This will clone line. For repalce and extractonly.
pub trait ReplaceLine {
    fn replace_line(&self, line: &str,) -> (bool, String);
}

/// for match mod
pub struct LineMatcher  {
    pub matcher: Box<dyn PatternMatch>,
}

/// did not clone line inside, but return an empty String
impl <'a> ReplaceLine for LineMatcher {
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        return (self.matcher.contain(line), line.to_owned() )
    }
}


/// for  extract mod
pub struct LineExtractor  {
    pub matcher: Box<dyn PatternMatch>,
}

impl <'a> ReplaceLine for LineExtractor {
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        match self.matcher.extract(line) {
            Some(v) => (true, v.to_owned()),
            None => (false, line.to_owned()),
        }
    }
}

/// for replace mod
pub struct LineReplacer <'a>{
    pub matcher: Box<dyn PatternMatch>,
    pub substitute: &'a str,
    pub times: usize,
}


impl <'a> ReplaceLine for LineReplacer <'a>{
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        self.matcher.replace(line, self.substitute, self.times)
    }
}

