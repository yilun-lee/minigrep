pub mod grep;
pub mod utils;

use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};

use self::{
    grep::handler::{Grep, GrepGroup},
    utils::{
        glober::PathGlober,
        logger::PrintBuffer,
        reader::{FileReader, MyErrors},
    },
};
/// main loop for grep a file
/// * file_reader: [FileReader](FileReader) object, read file by line.
/// * grep_group: object with [Grep](Grep), match a line by multiple regular
///   expression operation.
/// * print_buffer: [PrintBuffer](PrintBuffer) object, read file and put it into
///   buffer
/// * -> return
///     * [PrintBuffer](PrintBuffer) object filled with matched line.
pub fn main_loop(
    mut file_reader: FileReader,
    grep_group: &impl Grep,
    mut print_buffer: PrintBuffer,
) -> Result<PrintBuffer, anyhow::Error> {
    let behind_size = file_reader.behind_size;

    let mut whithin_flag: bool = false;
    let mut line_after_match: i32 = 0;
    let mut match_times = 0;

    loop {
        // handle different error https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560
        let line: &str = match file_reader.next() {
            Ok(v) => v,
            // if my custom EOF error
            Err(err) if err.downcast_ref() == Some(&MyErrors::EndOfFile) => {
                return Ok(print_buffer)
            }
            Err(err) => return Err(err),
        };

        let (match_flag, matched_line) = grep_group.grep_one_line(line);
        if match_flag {
            if match_times == 0 && !print_buffer.file_path_flag {
                print_buffer.push(
                    format!("\u{1b}[32m{}\u{1b}[39m:", &file_reader.file_path),
                    -1,
                );
            };

            if !whithin_flag {
                file_reader.print_buffer(&mut print_buffer);
                whithin_flag = true;
            }
            print_buffer.push(matched_line, file_reader.cc);
            line_after_match = 0;

            match_times += 1;
        } else {
            line_after_match += 1;

            if line_after_match > behind_size {
                whithin_flag = false
            } else if whithin_flag {
                print_buffer.push(matched_line, file_reader.cc);
            }
        }
    }
}

/// argument for run
pub struct RunArg {
    pub ahead_size: i32,
    pub behind_size: i32,
    pub file_path_flag: bool,
    pub line_num_flag: bool,
}

type WorkerOutput = Result<PrintBuffer>;
pub struct ThreadWorker {
    // re group
    pub my_re: Arc<GrepGroup>,
    // some config
    pub run_arg: Arc<RunArg>,
    // recieve from main string, file path
    pub reciever: Receiver<Option<PathBuf>>,
    // send to log thread
    pub sender: Sender<WorkerOutput>,
}

impl ThreadWorker {
    // run in seperate thread
    pub fn run(&self) {
        loop {
            // receive
            for file_path in &self.reciever {
                let file_path = match file_path {
                    Some(v) => v,
                    None => return,
                };
                let file_path = file_path.into_os_string().into_string().unwrap();
                // new PrintBuffer
                let print_buffer = PrintBuffer::new(
                    self.run_arg.file_path_flag,
                    self.run_arg.line_num_flag,
                    file_path.clone(),
                );
                // new file reader and check file
                let file_reader = match FileReader::new(
                    file_path.clone(),
                    self.run_arg.ahead_size,
                    self.run_arg.behind_size,
                ) {
                    Ok(v) => v,
                    Err(v) => {
                        self.sender
                            .send(Err(anyhow!("read {} error: {v}", file_path)))
                            .unwrap();
                        continue;
                    }
                };
                // run
                let out: std::result::Result<PrintBuffer, anyhow::Error> =
                    main_loop(file_reader, &*self.my_re, print_buffer);
                // send
                self.sender.send(out).unwrap();
            }
        }
    }
}

pub fn run_single_thread(
    my_re: GrepGroup,
    run_arg: RunArg,
    file_pattern: &str,
    skip_hidden: bool,
    max_depth: usize,
) -> Result<()> {
    // drop after finished
    let (path_sender, path_receiver) = unbounded();
    PathGlober::new(file_pattern, skip_hidden, max_depth, path_sender)?;

    for file_path in path_receiver {
        let file_path = match file_path {
            Some(v) => v,
            None => {
                println!("{:?} not found", file_path);
                continue;
            }
        };
        let file_path = file_path.into_os_string().into_string().unwrap();
        // new PrintBuffer
        let print_buffer = PrintBuffer::new(
            run_arg.file_path_flag,
            run_arg.line_num_flag,
            file_path.clone(),
        );
        // new file reader and check file
        let file_reader =
            match FileReader::new(file_path.clone(), run_arg.ahead_size, run_arg.behind_size) {
                Ok(v) => v,
                Err(v) => {
                    println!("read {} error: {v}", file_path);
                    continue;
                }
            };
        // run
        let print_buffer = match main_loop(file_reader, &my_re, print_buffer) {
            Ok(v) => v,
            Err(e) => {
                println!("read {} error: {e}", file_path);
                continue;
            }
        };
        print_buffer.print_all();
    }
    Ok(())
}
