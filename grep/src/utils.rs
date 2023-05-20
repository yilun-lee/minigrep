
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::LinkedList;
use anyhow::{anyhow,Error};

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
}



impl <'a>  FileReader {
    /// A next function for custom iterator
    /// want to use borrow as output here so we implement Iterator instead of use std one.
    /// (ref link)[http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html#a-better-iolines]
    pub fn new(file_path: String) -> Result<FileReader, anyhow::Error> {

        let mut file_reader = FileReader {
            buf_reader: read_file(file_path)?,
            buffer: LinkedList::new(),
            line: String::new(),        
        };
        Ok(file_reader)
    }
    
    
    pub fn next (&'a mut self) -> Result<&'a str, anyhow::Error>{

        if let 0 = self.buf_reader.read_line(&mut self.line)?{
            return Err(anyhow!("EOF"))
        };
    
        self.buffer.push_back(self.line.clone());
        self.buffer.pop_front(); // empyt return None, so it is Ok

        Ok(&self.line)
    }



}











