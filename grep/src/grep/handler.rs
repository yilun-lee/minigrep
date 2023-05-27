

use std::borrow::Cow;

use super::{matcher::PatternMatch};
use super::str_const::{COLOER_RESET,RED_START};



/// trait for match a line and return bool, for match only
/// May be more efficinet
/// not used for now
pub trait MatchLine {
    fn match_line(&self, line: &str,) -> bool;
}

/// trait for match a line and return bool and line
/// This will clone line. For repalce and extractonly.
pub trait ReplaceLine {
    fn replace_line(&self, line: &str,) -> (bool, String);
}

/// for match mod
pub struct LineMatcher {
    pub matcher: Box<dyn PatternMatch>,
}

/// did not clone line inside, but return an empty String
impl <'a> ReplaceLine for LineMatcher {
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        return (self.matcher.contain(line), line.to_owned() )
    }
}

pub struct LineExtractor {
    pub matcher: Box<dyn PatternMatch>,
}

impl <'a> ReplaceLine for LineExtractor {
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        let match_vec = self.matcher.extract(line);
        let mut my_str = String::from("");
        let mut cc: usize = 0;
        for (i,j) in &match_vec{
            if cc >= *i {continue;}
            if cc == 0 {
                my_str = my_str + &line[*i..*j]
            }else {
                my_str = my_str + 
                "\n" +
                &line[*i..*j];
            };
            cc = *j;
        }
        if match_vec.len() ==  0{
            my_str = my_str + &line
        }

        return (match_vec.len()> 0, my_str )
    }
}


pub struct LinePainter {
    pub matcher: Box<dyn PatternMatch>,
}

impl <'a> ReplaceLine for LinePainter {
    fn replace_line(&self, line: &str, ) -> (bool, String) {
        let match_vec = self.matcher.extract(line);
        let mut my_str = String::from("");
        let mut cc: usize = 0;
        for (i,j) in &match_vec{
            if cc >= *i {continue;}
            my_str = my_str + 
                &line[cc..*i] +
                RED_START +
                &line[*i..*j] +
                COLOER_RESET;
            cc = *j;
        }
        if match_vec.len()> 0 && cc < line.len() { 
            my_str = my_str + &line[cc..];
        } else if match_vec.len() ==  0{
            my_str = my_str + &line
        }

        return (match_vec.len()> 0, my_str )
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
       match self.matcher.replace(line, self.substitute, self.times) {
            Cow::Borrowed(v) => return (false, line.to_owned()),
            Cow::Owned(v) => return (true, v),
       }
    }
}

