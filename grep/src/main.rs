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
use argparse::MiniGrepArg;

use std::env;
use grep::grep::handler::GrepGroup;
use grep::utils::reader::FileReader;
use grep::utils::logger::LinePrinter;
use grep::utils::glober::PathGlober;



/// main function for arg
fn main(){
    // parse arg
    let my_arg = match MiniGrepArg::new(env::args()) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Argument parse error!",v)
    };

    // create grep
    let my_re = GrepGroup::from_re_group(
        my_arg.expr, my_arg.extract_expr, my_arg.replace_expr, my_arg.replacer, my_arg.replace_times, 
        my_arg.ignorecase, my_arg.color_flag)
        .expect("GrepGroup build failed");

    let line_printer: LinePrinter = LinePrinter{ line_num_flag: my_arg.line_num_flag, 
        file_path_flag: my_arg.file_path_flag, };
    
    let my_path: PathGlober = PathGlober::new(&my_arg.file_path, my_arg.skip_hidden)
        .expect("PathGlober run failed");

    for file_path in my_path {
        let file_reader = match FileReader::new(file_path.clone(), my_arg.ahead_size, my_arg.behind_size){
            Ok(v) => v,
            Err(v) => {
                eprintln!("read {} error: {v}", file_path); continue;}
        };
        let out =  main_loop(
            file_reader, 
            &my_re,
            line_printer.clone(),
        );
        match out {
            Ok(v) => v,
            Err(v) => {
                eprintln!("match {} error: {v}", file_path); continue;}
        }
    }

}


// {{}}}
// {{}}} }