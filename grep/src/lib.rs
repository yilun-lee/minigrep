
pub mod utils;
pub mod grep;
mod test;
pub mod argparse;

use crate::grep::matcher::PatternMatch;
use grep::handler::{LineExtractor,LinePainter,ReplaceLine};
use utils::{FileReader, MyErrors, LinePrinter, };

/// main loop for grep a file
/// * file_reader: [FileReader](FileReader) object, read file by line.
/// * line_handler: struct with [ReplaceLine](ReplaceLine) trait, match pattern and handle the match reuslt.
pub fn main_loop(
        mut file_reader: FileReader, 
        matcher: impl PatternMatch + 'static,
        match_only_flag: bool,
        line_prinrer: LinePrinter,
        ) -> Result<(), anyhow::Error> {
    
    let behind_size = file_reader.behind_size.clone();

    let mut whithin_flag = false;
    let mut line_after_match = 0;
    let mut match_times = 0;

    //if match_only_flag {
    //} else {
    //    let line_handler = LinePainter {matcher: Box::new(matcher)};
    //}
    //let line_handler = LineExtractor {matcher: Box::new(matcher)};
    let line_handler = LinePainter {matcher: Box::new(matcher)};

    loop {

        // handle different error https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560
        let line: &str = match file_reader.next() {
            Ok(v) => v,
            // if my custom EOF error
            Err(err) if err.downcast_ref() == Some(&MyErrors::EndOfFile) 
                => return Ok(()),
            Err(err) => return Err(err),
        };
        
        let (match_flag, matched_line) = &line_handler.replace_line(line);
        if *match_flag {
            if match_times == 0 { println!("\u{1b}[32m{}\u{1b}[39m:",&file_reader.file_path); };

            if ! whithin_flag{
                file_reader.print_buffer(&line_prinrer);
                whithin_flag = true;
            } 
            line_prinrer.print(&matched_line, file_reader.cc as usize, &file_reader.file_path);
            line_after_match = 0;

            match_times += 1;
        } else {

            line_after_match += 1;
            
            if line_after_match > behind_size {
                whithin_flag = false
            
            } else if whithin_flag {
                line_prinrer.print(&matched_line, file_reader.cc as usize, &file_reader.file_path);
            }
        }

    }
}



