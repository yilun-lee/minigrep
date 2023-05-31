pub trait LinePrint {
    fn print(&self, line: &str, num: usize, file_path: &str);
}

/// for weather print value with num and file_name
#[derive(Clone, Debug, Copy)]
pub struct LinePrinter {
    pub line_num_flag: bool,
    pub file_path_flag: bool,
}

/// print value
impl LinePrint for LinePrinter {
    fn print(&self, line: &str, num: usize, file_path: &str) {
        for i in line.split('\n') {
            if self.file_path_flag {
                print!("\u{1b}[32m{}\u{1b}[39m: ", file_path)
            }

            if self.line_num_flag {
                print!("\u{1b}[34m{}\u{1b}[39m: ", num)
            }

            println!("{}", i)
        }
    }
}

pub struct PrintBuffer {
    num_line: Vec<(i32, String)>,
    pub file_path: String,
    pub file_path_flag: bool,
    line_num_flag: bool,
    pub cc: usize,
}

impl PrintBuffer {
    pub fn new(file_path_flag: bool, line_num_flag: bool, file_path: String) -> PrintBuffer {
        PrintBuffer {
            num_line: vec![],
            file_path: file_path,
            file_path_flag: file_path_flag,
            line_num_flag: line_num_flag,
            cc: 0,
        }
    }

    pub fn push(&mut self, line: String, num: i32) {
        self.num_line.push((num, line));
        self.cc += 1;
    }

    fn print(&self, line: &str, num: i32) {
        // special case for header line
        if num < 0 {
            println!("{}", line);
            return;
        }
        for i in line.split('\n') {
            if self.file_path_flag {
                print!("\u{1b}[32m{}\u{1b}[39m: ", &self.file_path)
            }

            if self.line_num_flag {
                print!("\u{1b}[34m{}\u{1b}[39m: ", num)
            }

            println!("{}", i)
        }
    }

    pub fn print_all(&self) {
        self.num_line
            .iter()
            .for_each(|(num, line)| self.print(line, *num))
    }
}
