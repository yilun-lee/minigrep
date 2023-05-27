//! test code 
//! 
//! ```bash
//! /Users/sox/CODE/minigrep/target/debug/grep -i "AA" -A 2 /Users/sox/CODE/minigrep/example/test.txt
//! ```



// argparse is only used here 
mod argparse;


// use lib here
// main should access other module through lib
use grep::main_loop;
use grep::utils::{FileReader, LinePrinter, PathGlober};
use argparse::MiniGrepArg;
use grep::grep::matcher::RegexMatcher;
use grep::grep::handler::{LinePainter,LineExtractor,ReplaceLine};

use std::env;



/// main function for arg
fn main(){
    // parse arg
    let my_arg = match MiniGrepArg::new(env::args()) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Argument parse error!",v)
    };


    // create grep
    let my_re: RegexMatcher = match RegexMatcher::new(&my_arg.expression, my_arg.ignorecase){
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Regex create error!",v)
    };

    let line_printer: LinePrinter = LinePrinter{ line_num_flag: my_arg.line_num_flag, 
        file_path_flag: my_arg.file_path_flag, };

    let my_path: PathGlober = PathGlober::new(&my_arg.file_path, my_arg.skip_hidden).unwrap();

    for file_path in my_path {
        let file_reader = FileReader::new(file_path, my_arg.ahead_size, my_arg.behind_size).unwrap();
        let out =  main_loop(
            file_reader, 
            my_re.clone(),
            my_arg.match_only_flag,
            line_printer.clone(),
        );
        match out {
            Ok(v) => v,
            Err(v) => panic!("{}:\n {}", "Match error!",v)
        }
    }

}


