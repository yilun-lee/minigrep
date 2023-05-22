
pub mod utils;
pub mod grep;
mod test;

use grep::handler::ReplaceLine;

use utils::{FileReader, MyErrors};

/// main loop for grep a file
/// * file_reader: [FileReader](FileReader) object, read file by line.
/// * line_handler: struct with [ReplaceLine](ReplaceLine) trait, match pattern and handle the match reuslt.
fn main_loop<'a>(
        mut file_reader: FileReader, 
        line_handler: impl ReplaceLine,
        ) -> Result<(), anyhow::Error> {
    
    let behind_size = file_reader.behind_size.clone();

    let mut whithin_flag = false;
    let mut line_after_match = 0;

    loop {

        // handle different error https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560
        let line: &str = match file_reader.next() {
            Ok(v) => v,
            Err(err) if err.downcast_ref() == Some(&MyErrors::EndOfFile) 
                => return Ok(()),
            Err(err) => return Err(err),
        };
        
        let (match_flag, matched_line) = line_handler.replace_line(line);
        if match_flag {
            if ! whithin_flag{
                file_reader.print_buffer();
                whithin_flag = true;
            } 
            println!("{}", matched_line);
            line_after_match = 0;

        } else {

            line_after_match += 1;
            
            if line_after_match > behind_size {
                whithin_flag = false
            
            } else if whithin_flag {
                println!("{}", matched_line);
            }
        }

    }
}




fn main(){
    println!("Hello World");
}
