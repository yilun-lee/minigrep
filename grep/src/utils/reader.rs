use anyhow::anyhow;
use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use thiserror;

use super::logger::LinePrint;

/// My own error
///
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum MyErrors {
    ///
    EndOfFile,
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

pub struct FileReader {
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

impl<'a> FileReader {
    /// create a new FileReader instance
    /// * `file_path` - File path
    /// * `ahead_size` - buffer size ahead
    /// * `behind_size` -  size after match line
    pub fn new(
        file_path: String,
        ahead_size: i32,
        behind_size: i32,
    ) -> Result<FileReader, anyhow::Error> {
        let file_reader = FileReader {
            file_path: file_path.clone(),
            buf_reader: read_file(file_path)?,
            buffer: LinkedList::new(),
            line: String::new(),

            cc: 0,
            ahead_size: ahead_size,
            behind_size: behind_size,
        };
        Ok(file_reader)
    }

    /// A next function for custom iterator
    /// want to use borrow as output here so we implement Iterator instead of use std one.
    /// (ref link)`<http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html#a-better-iolines>`
    pub fn next(&'a mut self) -> Result<&'a str, anyhow::Error> {
        // push the previous line
        if self.cc > 0 {
            self.buffer.push_back(self.line.clone());
        };

        self.cc += 1;
        if self.cc > self.ahead_size + 1 {
            self.buffer.pop_front();
        }

        self.line = match self.buf_reader.next() {
            Some(v) => v?,
            None => return Err(anyhow!(MyErrors::EndOfFile)),
        };

        Ok(&self.line)
    }

    /// print all ahead buffer
    pub fn print_buffer(&self, line_printer: &impl LinePrint) {
        let mut cc = self.cc - 1;
        for i in &self.buffer {
            line_printer.print(i, cc as usize, &self.file_path);
            cc -= 1;
        }
    }
}
