use std::borrow::Cow;

use anyhow::Result;

use super::{
    matcher::{PatternMatch, RegexMatcher},
    str_const::{COLOER_RESET, RED_START},
};

pub trait Grep {
    fn grep_one_line(&self, line: &str) -> (bool, String);
}

pub struct GrepGroup {
    match_list: Vec<Box<dyn PatternMatch>>,
    color_flag: bool,

    extract_list: Vec<Box<dyn PatternMatch>>,

    replace_list: Vec<Box<dyn PatternMatch>>,
    replace_tobe: Vec<String>,
    replace_times: usize,
}

impl Grep for GrepGroup {
    fn grep_one_line(&self, line: &str) -> (bool, String) {
        let mut match_flag: bool = false;
        let mut my_str: String = line.to_owned();

        // match
        if !self.match_list.is_empty() {
            (match_flag, my_str) = self.match_line(line);
            if !match_flag {
                return (false, my_str);
            };
        };
        // extract
        if !self.extract_list.is_empty() {
            (_, my_str) = self.extract_line(line);
        };
        // replace
        if !self.replace_list.is_empty() {
            my_str = self.replace_line(&my_str);
        };

        (match_flag, my_str)
    }
}

/// pub method for GrepGroup
impl GrepGroup {
    pub fn from_re_group(
        expr: Vec<String>,
        extract_expr: Vec<String>,
        replace_expr: Vec<String>,
        replacer: Vec<String>,
        replace_times: usize,
        ignorecase: bool,
        color_flag: bool,
    ) -> Result<GrepGroup> {
        let mut match_list: Vec<Box<dyn PatternMatch>> = vec![];
        for i in &expr {
            let my_re = RegexMatcher::new(i, ignorecase)?;
            match_list.push(Box::new(my_re));
        }

        let mut extract_list: Vec<Box<dyn PatternMatch>> = vec![];
        for i in &extract_expr {
            let my_re: RegexMatcher = RegexMatcher::new(i, ignorecase)?;
            extract_list.push(Box::new(my_re));
        }

        let mut replace_list: Vec<Box<dyn PatternMatch>> = vec![];
        for i in &replace_expr {
            let my_re = RegexMatcher::new(i, ignorecase)?;
            replace_list.push(Box::new(my_re));
        }

        let my_grep_group = GrepGroup {
            match_list,
            color_flag,

            extract_list,

            replace_list,
            replace_tobe: replacer,
            replace_times,
        };
        Ok(my_grep_group)
    }

    pub fn match_line(&self, line: &str) -> (bool, String) {
        if !self.color_flag {
            let mut flag: bool = true;
            for i in &self.match_list {
                flag = i.contain(line);
                if !flag {
                    break;
                }
            }
            (flag, line.to_owned())
        } else {
            let target_vec = self.aggregate_extract(line, &self.match_list);
            if !target_vec.iter().any(|i: &bool| *i) {
                return (false, line.to_owned());
            }
            let my_str: String = self.paint_line(line, target_vec, false);
            (true, my_str)
        }
    }

    pub fn extract_line(&self, line: &str) -> (bool, String) {
        let target_vec = self.aggregate_extract(line, &self.extract_list);
        if !target_vec.iter().any(|i: &bool| *i) {
            return (false, line.to_owned());
        }
        let my_str = self.paint_line(line, target_vec, true);
        (true, my_str)
    }

    pub fn replace_line(&self, line: &str) -> String {
        let mut my_str: String = line.to_owned();
        let mut cc = 0;
        for i in &self.replace_list {
            match i.replace(&my_str, &self.replace_tobe[cc], self.replace_times) {
                Cow::Borrowed(_) => continue,
                Cow::Owned(v) => my_str = v,
            }
            cc += 1
        }
        my_str
    }
}

/// private method for GrepGroup
impl GrepGroup {
    fn aggregate_extract(
        &self,
        line: &str,
        extract_list: &Vec<Box<dyn PatternMatch>>,
    ) -> Vec<bool> {
        let mut target_vec: Vec<bool> = vec![false; line.len()];

        for matcher in extract_list {
            let match_vec = matcher.extract(line);
            for (i, j) in match_vec {
                for k in i..j {
                    target_vec[k] = true;
                }
            }
        }

        target_vec
    }

    fn paint_line(&self, line: &str, target_vec: Vec<bool>, extract_flag: bool) -> String {
        let mut my_str = String::from("");
        let mut cc: usize = 1;
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut last_end: usize = 0;
        for flag in &target_vec {
            if *flag {
                // only end mv forward
            } else {
                if start != end {
                    if !extract_flag {
                        my_str = my_str
                            + &line[last_end..start]
                            + RED_START
                            + &line[start..end]
                            + COLOER_RESET;
                        last_end = end;
                    } else if my_str.is_empty() {
                        my_str += &line[start..end];
                    } else {
                        my_str = my_str + "\n" + &line[start..end]
                    }
                }
                start = cc;
            }
            end = cc;
            cc += 1;
        }

        if last_end <= cc && !extract_flag {
            if start != end {
                my_str =
                    my_str + &line[last_end..start] + RED_START + &line[start..] + COLOER_RESET;
            } else {
                my_str += &line[last_end..];
            }
        }
        my_str
    }
}
