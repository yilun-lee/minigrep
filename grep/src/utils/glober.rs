
use std::{path::{PathBuf}, fs};
use glob::{glob};
use anyhow::anyhow;



pub struct PathGlober {
    pathbuf_vec: Vec<PathBuf>,
    skip_hidden: bool,
}

impl PathGlober {
    pub fn new(file_pattern: &str, skip_hidden: bool) -> Result<PathGlober, anyhow::Error>{
        let mut path_glober = PathGlober {
            pathbuf_vec: vec![],
            skip_hidden: skip_hidden,
        };
        path_glober.search_path(file_pattern)?;
        if path_glober.pathbuf_vec.len() == 0 {
            return Err(anyhow!("No files found!"))
        }
        return Ok(path_glober);
    }

    fn search_path(&mut self, file_pattern: &str, ) -> Result<(), anyhow::Error>{
        let entry = glob(file_pattern)?;
        for i in entry{
            let my_pathbuf: PathBuf = fs::canonicalize(i?)?;
            let my_path = my_pathbuf.as_path();
            let my_path_str = my_path
                .file_name().ok_or(anyhow!("Get filename failded"))?
                .to_str().ok_or(anyhow!("filename to str failded"))?;
            if my_path_str.starts_with(".") && self.skip_hidden {continue;};

            if my_path.is_dir() {
                let dirpath = match my_path.parent(){
                    Some(v) => {
                        let vv = v.to_str().ok_or(anyhow!("filename to str failded"))?;
                        format!("{}/{}/*", vv, my_path_str) },
                    None =>  format!("{}/*", my_path_str),
                };
                self.search_path(&dirpath)?;
            } else {
                self.pathbuf_vec.push(my_pathbuf);
            }
        }
        Ok(())
    }

    fn read_one_entry(&self, my_path: PathBuf) -> Result<String, anyhow::Error>{
        let file_path: String = match my_path.clone()
            .into_os_string().into_string(){
            Ok(v) => v,
            Err(_) => return Err( anyhow!("pathbuf to os string error")),
        };

        return Ok(file_path);

    }

}


impl Iterator for PathGlober{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let pathbuf= self.pathbuf_vec.pop()?;
        let my_out = match self.read_one_entry(pathbuf) {
            Ok(v) => v,
            Err(_) => return None
        };  

        Some(my_out)
    }
}
