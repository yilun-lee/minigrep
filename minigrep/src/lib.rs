#![doc = include_str!("../../README.md")]

pub mod argparse;
pub mod grep;
mod test;
pub mod utils;

use grep::handler::Grep;

use utils::logger::LinePrint;
use utils::reader::{FileReader, MyErrors};

/// main loop for grep a file
/// * file_reader: [FileReader](FileReader) object, read file by line.
pub fn main_loop(
    mut file_reader: FileReader,
    grep_group: &impl Grep,
    line_prinrer: impl LinePrint,
) -> Result<(), anyhow::Error> {
    let behind_size = file_reader.behind_size;

    let mut whithin_flag: bool = false;
    let mut line_after_match = 0;
    let mut match_times = 0;

    loop {
        // handle different error https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560
        let line: &str = match file_reader.next() {
            Ok(v) => v,
            // if my custom EOF error
            Err(err) if err.downcast_ref() == Some(&MyErrors::EndOfFile) => return Ok(()),
            Err(err) => return Err(err),
        };

        let (match_flag, matched_line) = &grep_group.grep_one_line(line);
        if *match_flag {
            if match_times == 0 {
                println!("\u{1b}[32m{}\u{1b}[39m:", &file_reader.file_path);
            };

            if !whithin_flag {
                file_reader.print_buffer(&line_prinrer);
                whithin_flag = true;
            }
            line_prinrer.print(
                matched_line,
                file_reader.cc as usize,
                &file_reader.file_path,
            );
            line_after_match = 0;

            match_times += 1;
        } else {
            line_after_match += 1;

            if line_after_match > behind_size {
                whithin_flag = false
            } else if whithin_flag {
                line_prinrer.print(
                    matched_line,
                    file_reader.cc as usize,
                    &file_reader.file_path,
                );
            }
        }
    }
}