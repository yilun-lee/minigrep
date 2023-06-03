pub mod gitclone;
pub mod mygrep;
mod test;

use std::{collections::HashMap, fs};

use anyhow::Result;
use gitclone::{GitClone, GitCloner};
use mygrep::{FileMatch, MiniGrepMatcher};
use nanoid::nanoid;

pub struct GitGrepArg {
    output_path: String,
    grep_matcher: Box<dyn FileMatch>,
    cache_table: HashMap<String, String>,
}

impl GitGrepArg {
    pub fn new(
        output_path: String,
        expr: Vec<String>,
        ahead_size: i32,
        behind_size: i32,
    ) -> GitGrepArg {
        let grep_matcher = MiniGrepMatcher::new(expr, ahead_size, behind_size);
        GitGrepArg {
            grep_matcher: Box::new(grep_matcher),
            output_path: output_path,
            cache_table: HashMap::new(),
        }
    }

    pub fn clone_match(&mut self, git_url: &str) -> Result<()> {
        let save_dir: String = match self.cache_table.get(git_url) {
            None => {
                // create new id
                let uuid: String = nanoid!(10);
                let save_dir: String = format!("{}/{}", &self.output_path, uuid);
                // clone
                let mut git_cloner = GitCloner::new(vec![]);
                git_cloner.git_clone(git_url, &save_dir)?;
                // insert
                self.cache_table
                    .insert(git_url.to_owned(), save_dir.clone());
                save_dir
            }
            Some(v) => v.clone(),
        };

        self.grep_matcher.match_job(save_dir)?;

        Ok(())
    }

    pub fn free_cache(&mut self) -> Result<()> {
        for (_, v) in &self.cache_table {
            fs::remove_dir_all(v)?;
        }
        self.cache_table = HashMap::new();
        Ok(())
    }
}
