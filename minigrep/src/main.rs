//! test code
//!
//! ```bash
//! /Users/sox/CODE/minigrep/target/debug/grep -i "AA" -A 2 /Users/sox/CODE/minigrep/example/test.txt
//! ```

// argparse is only used here
mod argparse;

// use lib here
// main should access other module through lib
use std::env;

use argparse::MiniGrepArg;
use minigrep::{
    glober_thread, parallel_match,
    runner::{grep::handler::GrepGroup, run_single_thread, RunArg},
};

/// main function for arg
fn main() {
    // parse arg
    let my_arg = match MiniGrepArg::new(env::args()) {
        Ok(v) => v,
        Err(v) => panic!("{}:\n {}", "Argument parse error!", v),
    };

    // create grep
    let my_re = GrepGroup::from_re_group(
        my_arg.expr,
        my_arg.extract_expr,
        my_arg.replace_expr,
        my_arg.replacer,
        my_arg.replace_times,
        my_arg.ignorecase,
        my_arg.color_flag,
    )
    .expect("GrepGroup build failed");

    // run arg
    let run_arg = RunArg {
        ahead_size: my_arg.ahead_size,
        behind_size: my_arg.behind_size,
        file_path_flag: my_arg.file_path_flag,
        line_num_flag: my_arg.line_num_flag,
    };

    if my_arg.thread_num > 1 {
        // glober
        let (glober_thread, path_receiver) = glober_thread(
            my_arg.file_path,
            my_arg.skip_hidden,
            my_arg.max_depth,
            my_arg.thread_num,
        );

        let matcher_thread_vec =
            parallel_match(run_arg, my_re, my_arg.thread_num, path_receiver).unwrap();

        // wait for end
        glober_thread.join().unwrap();
        for i in matcher_thread_vec {
            i.join().unwrap();
        }
    } else {
        run_single_thread(
            my_re,
            run_arg,
            &my_arg.file_path,
            my_arg.skip_hidden,
            my_arg.max_depth,
        )
        .unwrap();
    }
}

// {{}}}
// {{}}} }
