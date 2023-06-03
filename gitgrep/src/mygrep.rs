use anyhow::{anyhow, Result};
use minigrep::{glober_thread, parallel_match, GrepGroup, RunArg};

pub trait FileMatch {
    fn match_job(&self, file_path: String) -> Result<()>;
}

pub struct MiniGrepMatcher {
    ahead_size: i32,
    behind_size: i32,
    line_num_flag: bool,
    file_path_flag: bool,
    color_flag: bool,

    expr: Vec<String>,
    replace_times: usize,
    ignorecase: bool,

    skip_hidden: bool,
    max_depth: usize,
    thread_num: usize,
}

impl Default for MiniGrepMatcher {
    fn default() -> Self {
        MiniGrepMatcher {
            ahead_size: 0,
            behind_size: 0,
            line_num_flag: true,
            file_path_flag: false,
            color_flag: true,

            expr: vec![],
            replace_times: 0,
            ignorecase: true,

            skip_hidden: false,
            max_depth: 10,
            thread_num: 5,
        }
    }
}

impl MiniGrepMatcher {
    pub fn new(expr: Vec<String>, ahead_size: i32, behind_size: i32) -> MiniGrepMatcher {
        MiniGrepMatcher {
            expr,
            ahead_size,
            behind_size,
            ..MiniGrepMatcher::default()
        }
    }
}

// file_path
// expr
// ahead_size
// behind_size
impl FileMatch for MiniGrepMatcher {
    fn match_job(&self, file_path: String) -> Result<()> {
        // create grep
        let my_re = GrepGroup::from_re_group(
            self.expr.clone(),
            vec![],
            vec![],
            vec![],
            self.replace_times,
            self.ignorecase,
            self.color_flag,
        )?;

        // run arg
        let run_arg = RunArg {
            ahead_size: self.ahead_size,
            behind_size: self.behind_size,
            file_path_flag: self.file_path_flag,
            line_num_flag: self.line_num_flag,
        };

        // glober
        let (glober_thread, path_receiver) =
            glober_thread(file_path, self.skip_hidden, self.max_depth, self.thread_num);

        let mut matcher_thread_vec =
            parallel_match(run_arg, my_re, self.thread_num, path_receiver)?;

        // wait for end
        matcher_thread_vec.push(glober_thread);
        for i in matcher_thread_vec {
            match i.join() {
                Ok(_) => (),
                // we need to cast dyn any to string
                // since it is option, we have to print it in debug mod
                Err(e) => return Err(anyhow!("{:?}", e.downcast_ref::<String>())),
            }
        }
        Ok(())
    }

    // {{}}}
    // {{}}} }
}
