//! test code
//!
//! ```bash
//! /Users/sox/CODE/minigrep/target/debug/grep -i "AA" -A 2 /Users/sox/CODE/minigrep/example/test.txt
//! ```

// argparse is only used here
mod argparse;

// use lib here
// main should access other module through lib
use anyhow::{anyhow, Result};
use argparse::MiniGrepArg;
use crossbeam::channel::unbounded;
use minigrep::grep::handler::GrepGroup;
use minigrep::main_loop;
use minigrep::utils::glober::PathGlober;
use minigrep::utils::logger::LinePrinter;
use minigrep::utils::reader::FileReader;
use std::{env, path::PathBuf, sync::Arc, thread};

fn simple_match(
    my_path: PathGlober,
    ahead_size: i32,
    behind_size: i32,
    my_re: GrepGroup,
    line_printer: LinePrinter,
) {
    for file_path in my_path {
        let file_reader = match FileReader::new(file_path.clone(), ahead_size, behind_size) {
            Ok(v) => v,
            Err(v) => {
                eprintln!("read {} error: {v}", file_path);
                continue;
            }
        };
        let out = main_loop(file_reader, &my_re, line_printer.clone());
        match out {
            Ok(v) => v,
            Err(v) => {
                eprintln!("match {} error: {v}", file_path);
                continue;
            }
        }
    }
}

fn parallel_match(
    mut my_path: PathGlober,
    ahead_size: i32,
    behind_size: i32,
    my_re: GrepGroup,
    line_printer: LinePrinter,
) {
    let (tx, rx) = unbounded();
    let mut children = Vec::new();
    let my_re = Arc::new(my_re);
    let line_printer = Arc::new(line_printer);

    loop {
        let file_path = match my_path.next() {
            Some(v) => v,
            None => break,
        };
        let tx = tx.clone();
        let my_re_inside = Arc::clone(&my_re);
        let line_printer_inside = Arc::clone(&line_printer);

        children.push(thread::spawn(move || {
            let file_reader = match FileReader::new(file_path.clone(), ahead_size, behind_size) {
                Ok(v) => v,
                Err(v) => {
                    tx.send(Err(anyhow!("read {} error: {v}", file_path)))
                        .unwrap();
                    return;
                }
            };
            let out = main_loop(file_reader, &*my_re_inside, *line_printer_inside);
            tx.send(out).unwrap();
            return;
        }));
    }

    drop(tx);
    for out in &rx {
        match out {
            Ok(v) => v,
            Err(v) => {
                eprintln!("{}", v);
            }
        }
    }
}

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

    let line_printer: LinePrinter = LinePrinter {
        line_num_flag: my_arg.line_num_flag,
        file_path_flag: my_arg.file_path_flag,
    };

    let mut my_path: PathGlober =
        PathGlober::new(&my_arg.file_path, my_arg.skip_hidden, my_arg.max_depth)
            .expect("PathGlober run failed");

    parallel_match(
        my_path,
        my_arg.ahead_size,
        my_arg.behind_size,
        my_re,
        line_printer,
    );
}

// {{}}}
// {{}}} }
