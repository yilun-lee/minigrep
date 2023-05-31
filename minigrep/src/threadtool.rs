use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use crossbeam::channel::{Receiver, Sender};
use minigrep::grep::handler::GrepGroup;
use minigrep::main_loop;
use minigrep::utils::logger::PrintBuffer;
use minigrep::utils::reader::FileReader;

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
                let out = main_loop(file_reader, &*self.my_re, print_buffer);
                // send
                self.sender.send(out).unwrap();
            }
        }
    }
}
