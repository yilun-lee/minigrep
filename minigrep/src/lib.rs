#![doc = include_str!("../../README.md")]

pub mod argparse;
pub mod runner;
mod test;

use std::{path::PathBuf, sync::Arc, thread, thread::JoinHandle};

use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver};
pub use runner::{grep::handler::GrepGroup, main_loop, run_single_thread, RunArg};
use runner::{utils::glober::PathGlober, ThreadWorker};

pub fn glober_thread(
    file_path: String,
    skip_hidden: bool,
    max_depth: usize,
    thread_num: usize,
) -> (JoinHandle<()>, Receiver<Option<PathBuf>>) {
    let (path_sender, path_receiver) = unbounded();
    let my_thread: JoinHandle<()> = thread::spawn(move || {
        let path_sender = path_sender.clone();
        // run glober
        PathGlober::new(&file_path, skip_hidden, max_depth, path_sender.clone())
            .expect("PathGlober run failed");
        // turn off all tread after glob
        for _ in 0..thread_num {
            path_sender.send(None).unwrap();
        }
    });
    (my_thread, path_receiver)
}

pub fn parallel_match(
    run_arg: RunArg,
    my_re: GrepGroup,
    thread_num: usize,
    path_receiver: Receiver<Option<PathBuf>>,
) -> Result<Vec<JoinHandle<()>>> {
    let (line_sender, line_receiver) = unbounded();
    let my_re = Arc::new(my_re);
    let run_arg = Arc::new(run_arg);

    // run worker
    let mut thread_vec: Vec<JoinHandle<()>> = Vec::new();
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
    Ok(thread_vec)
}
