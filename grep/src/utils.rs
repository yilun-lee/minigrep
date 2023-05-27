
use std::io::{self, BufRead, BufReader, Lines};
use std::fs::{File, self};
use std::collections::LinkedList;
use std::path::{PathBuf, Path};
use anyhow::anyhow;
use thiserror;
use glob::{glob, Paths};

/// My own error
/// 
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum MyErrors {
    /// 
    EndOfFile
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndOfFile => write!(f, "File EOF reached"),
        }
    }
}

type LineBuf = Lines<BufReader<File>>;
/// read file and retuen buffer reader
pub fn read_file(file_path: String) -> io::Result<LineBuf> {
    let f: File = File::open(file_path)?;
    let cursor: BufReader<File> = BufReader::new(f);
    Ok(cursor.lines())
}



pub struct  FileReader {
    pub file_path: String,
    pub buf_reader: LineBuf,
    pub buffer: LinkedList<String>,
    pub line: String,
    
    /// line number index
    pub cc: i32, 
    /// buffer size, the size to show before matched line
    pub ahead_size: i32,
    /// the size to show after matched line
    /// pub because it will be used in main
    pub behind_size: i32,
}

impl <'a>  FileReader {
    /// create a new FileReader instance
    /// * `file_path` - File path
    /// * `ahead_size` - buffer size ahead 
    /// * `behind_size` -  size after match line 
    pub fn new(file_path: String, ahead_size: i32, behind_size: i32) -> Result<FileReader, anyhow::Error> {

        let file_reader = FileReader {
            file_path: file_path.clone(),
            buf_reader: read_file(file_path)?,
            buffer: LinkedList::new(),
            line: String::new(),

            cc: 0,
            ahead_size: ahead_size,
            behind_size: behind_size
        };
        Ok(file_reader)
    }
    
    /// A next function for custom iterator
    /// want to use borrow as output here so we implement Iterator instead of use std one.
    /// (ref link)`<http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html#a-better-iolines>`
    pub fn next (&'a mut self) -> Result<&'a str, anyhow::Error>{

        // push the previous line
        if self.cc > 0 {
            self.buffer.push_back(self.line.clone());
        };

        self.cc += 1;
        if self.cc > self.ahead_size + 1 {
            self.buffer.pop_front();} 

        self.line = match self.buf_reader.next() {
            Some(v) => v?,
            None => return Err(anyhow!(MyErrors::EndOfFile)),
        };
    
        Ok(&self.line)
    }

    /// print all ahead buffer
    pub fn print_buffer(&self, line_printer: &LinePrinter) {
        let mut cc = self.cc - 1;
        for i in &self.buffer {
            line_printer.print(i, cc as usize, &self.file_path);
            cc -= 1;
        };
    }
}



/// for weather print value with num and file_name
#[derive(Clone,Debug)]
pub struct LinePrinter {
    pub line_num_flag: bool, 
    pub file_path_flag: bool, 
}

/// print value
impl LinePrinter {
    pub fn print(&self, 
        line: &str, num: usize, file_path: &str, ) {

        if self.file_path_flag {
            print!("\u{1b}[32m{}\u{1b}[39m:", file_path)
        }
        
        if self.line_num_flag {
            print!("\u{1b}[34m{}\u{1b}[39m:", num)
        }

        println!("{}", line)

    }
}



pub struct PathGlober {
    file_pattern: String,
    pathbuf_vec: Vec<PathBuf>,
    skip_hidden: bool,
}

impl PathGlober {
    pub fn new(file_pattern: &str, skip_hidden: bool) -> Result<PathGlober, anyhow::Error>{
        let mut path_glober = PathGlober {
            file_pattern: file_pattern.to_string(),
            pathbuf_vec: vec![],
            skip_hidden: skip_hidden,
        };
        path_glober.search_path(file_pattern)?;
        return Ok(path_glober);
    }

    fn search_path(&mut self, file_pattern: &str, ) -> Result<(), anyhow::Error>{
        let entry = glob(file_pattern)?;
        for i in entry{
            let my_pathbuf = fs::canonicalize(i?)?;
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
