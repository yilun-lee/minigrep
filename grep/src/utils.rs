
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::LinkedList;
use anyhow::anyhow;

#[derive(Clone, Debug, PartialEq)]
enum MyErrors {
    EndOfFile
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndOfFile => write!(f, "File EOF reached"),
        }
    }
}

/// read file and retuen buffer reader
pub fn read_file(file_path: String) -> io::Result<BufReader<File>> {
    let f: File = File::open(file_path)?;
    let cursor = BufReader::new(f);
    Ok(cursor)
}



pub struct  FileReader {
    buf_reader: BufReader<File>,
    buffer: LinkedList<String>,
    line: String,

    cc: i32,
    buffer_size: i32,
}

impl <'a>  FileReader {
    pub fn new(file_path: String, buffer_size: i32) -> Result<FileReader, anyhow::Error> {

        let mut file_reader = FileReader {
            buf_reader: read_file(file_path)?,
            buffer: LinkedList::new(),
            line: String::new(),

            cc: 0,
            buffer_size: buffer_size,
        };
        Ok(file_reader)
    }
    
    /// A next function for custom iterator
    /// want to use borrow as output here so we implement Iterator instead of use std one.
    /// (ref link)`<http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html#a-better-iolines>`
    pub fn next (&'a mut self) -> Result<&'a str, anyhow::Error>{

        // handle different error https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560
        if let 0 = self.buf_reader.read_line(&mut self.line)?{
            return Err(anyhow!(MyErrors::EndOfFile))
        };
    
        self.buffer.push_back(self.line.clone());
        self.cc += 1;
        if self.cc > self.buffer_size {
            self.buffer.pop_front();} // empyt return None, so it is Ok

        Ok(&self.line)
    }

    fn print_buffer(&self) {
        for i in &self.buffer {
            println!("{}", i);
        };
    }
}


fn main_loop(mut file_reader: FileReader, a_num: i32, b_num: i32) -> Result<(), anyhow::Error> {

    let mut whithin_flag = false;
    let mut line_after_match = 0;

    loop {
        let line: &str = file_reader.next()?;

        if line.contains("AA") {
            if ! whithin_flag{
                for i in &file_reader.buffer {
                    println!("{}", i);
                };
                whithin_flag = true;
            
            } else {

                println!("{}", line);
            }

            line_after_match = 0;

        }else {

            line_after_match += 1;
            
            if line_after_match > b_num {
                whithin_flag = false
            
            } else {
                println!("{}", line);
            }
        }



    }
}








