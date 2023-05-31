//! test code
//!
//! ```bash
//! /Users/sox/CODE/minigrep/target/debug/grep -i "AA" -A 2 /Users/sox/CODE/minigrep/example/test.txt
//! ```

// argparse is only used here
mod argparse;
mod threadtool;

// use lib here
// main should access other module through lib
use anyhow::Result;
use argparse::MiniGrepArg;
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use minigrep::grep::handler::GrepGroup;
use minigrep::utils::glober::PathGlober;
use std::path::PathBuf;
use std::{env, sync::Arc, thread};
use threadtool::RunArg;
use threadtool::ThreadWorker;

fn glober_thread(
    file_path: String,
    skip_hidden: bool,
    max_depth: usize,
    thread_num: usize,
) -> Receiver<Option<PathBuf>> {
    let (path_sender, path_receiver) = unbounded();
    thread::spawn(move || {
        let path_sender = path_sender.clone();
        // run glober
        PathGlober::new(&file_path, skip_hidden, max_depth, path_sender.clone())
            .expect("PathGlober run failed");
        // turn off all tread after glob
        for _ in 0..thread_num {
            path_sender.send(None).unwrap();
        }
    });
    path_receiver
}

fn parallel_match(
    run_arg: RunArg,
    my_re: GrepGroup,
    thread_num: usize,
    path_receiver: Receiver<Option<PathBuf>>,
) -> Result<()> {
    let (line_sender, line_receiver) = unbounded();
    let my_re = Arc::new(my_re);
    let run_arg = Arc::new(run_arg);

    // run worker
    let mut thread_vec: Vec<_> = Vec::new();
    for _ in 0..thread_num {
        let thread_worker = ThreadWorker {
            my_re: my_re.clone(),
            run_arg: run_arg.clone(),
            reciever: path_receiver.clone(),
            // send to log thread
            sender: line_sender.clone(),
        };
        thread_vec.push(thread::spawn(move || {
            thread_worker.run();
            println!("Work complete");
        }))
    }

    drop(line_sender);
    drop(path_receiver);

    // print buffer
    for print_buffer in line_receiver {
        match print_buffer {
            Ok(v) => v.print_all(),
            Err(v) => {
                eprintln!("{}", v);
                continue;
            }
        }
    }
    Ok(())
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

    // run arg
    let run_arg = RunArg {
        ahead_size: my_arg.ahead_size,
        behind_size: my_arg.behind_size,
        file_path_flag: my_arg.file_path_flag,
        line_num_flag: my_arg.line_num_flag,
    };

    // glober
    let path_receiver = glober_thread(
        my_arg.file_path,
        my_arg.skip_hidden,
        my_arg.max_depth,
        my_arg.thread_num,
    );

    parallel_match(run_arg, my_re, my_arg.thread_num, path_receiver).unwrap();
}

// {{}}}
// {{}}} }
