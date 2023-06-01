use anyhow::anyhow;
use crossbeam::channel::Sender;
use glob::glob;
use std::{fs, path::PathBuf};

pub struct PathGlober {
    pub sender: Sender<Option<PathBuf>>,
    skip_hidden: bool,
    max_depth: usize,
}

impl PathGlober {
    pub fn new(
        file_pattern: &str,
        skip_hidden: bool,
        max_depth: usize,
        sender: Sender<Option<PathBuf>>,
    ) -> Result<PathGlober, anyhow::Error> {
        let mut path_glober = PathGlober {
            sender,
            skip_hidden,
            max_depth,
        };
        path_glober.search_path(file_pattern, 0)?;
        Ok(path_glober)
    }

    fn search_path(
        &mut self,
        file_pattern: &str,
        current_depth: usize,
    ) -> Result<(), anyhow::Error> {
        let entry = glob(file_pattern)?;
        for i in entry {
            let my_pathbuf: PathBuf = fs::canonicalize(i?)?;
            let my_path = my_pathbuf.as_path();
            let my_path_str: &str = my_path
                .file_name()
                .ok_or(anyhow!("Get filename failded"))?
                .to_str()
                .ok_or(anyhow!("filename to str failded"))?;
            if my_path_str.starts_with('.') && self.skip_hidden {
                continue;
            };

            if my_path.is_dir() && current_depth < self.max_depth {
                let dirpath = match my_path.parent() {
                    Some(v) => {
                        let vv = v.to_str().ok_or(anyhow!("filename to str failded"))?;
                        format!("{}/{}/*", vv, my_path_str)
                    }
                    None => format!("{}/*", my_path_str),
                };
                self.search_path(&dirpath, current_depth + 1)?;
            } else {
                self.sender.send(Some(my_pathbuf))?;
            }
        }
        Ok(())
    }

    fn _read_one_entry(&self, my_path: PathBuf) -> Result<String, anyhow::Error> {
        let file_path: String = match my_path.into_os_string().into_string() {
            Ok(v) => v,
            Err(_) => return Err(anyhow!("pathbuf to os string error")),
        };

        Ok(file_path)
    }
}
