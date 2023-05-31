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
