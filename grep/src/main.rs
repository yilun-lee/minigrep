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
use grep::utils::FileReader;
use argparse::MiniGrepArg;
use grep::grep::matcher::RegexMatcher;
use grep::grep::handler::LineMatcher;


use std::env;


/// main function for arg
fn main(){
    // parse arg
    let my_arg = match MiniGrepArg::new(env::args()) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Argument parse error!",v)
    };

    // read file
    let file_reader = match FileReader::new(my_arg.file_path, my_arg.ahead_size, my_arg.behind_size) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Read file error!",v)
    };

    // create grep
    let my_re = match RegexMatcher::new(&my_arg.expression, my_arg.ignorecase){
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Regex create error!",v)
    };
    let my_line_replacer = LineMatcher {
        matcher: Box::new(my_re),
    };

    // run
    match main_loop(file_reader, my_line_replacer){
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Match error!",v)
    };

}


