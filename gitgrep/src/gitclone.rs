use anyhow::anyhow;
use anyhow::Result;
use std::process::Command;

type OutErr = (String, String);
pub trait GitClone {
    fn git_clone(&mut self, git_url: &str, output_path: &str) -> Result<OutErr>;
}

pub struct GitCloner {
    args: Vec<String>,
    cmd: Command,
}

impl GitCloner {
    pub fn new(args: Vec<String>) -> GitCloner {
        GitCloner {
            args: args,
            cmd: Command::new("git"),
        }
    }
}

impl GitClone for GitCloner {
    fn git_clone(&mut self, git_url: &str, output_path: &str) -> Result<OutErr> {
        let output = self
            .cmd
            .arg("clone")
            .args(&self.args)
            .arg(git_url)
            .arg(output_path)
            .output()?;
        let out = String::from_utf8(output.stdout)?;
        let err = String::from_utf8(output.stderr)?;
        if !output.status.success() {
            return Err(anyhow!(
                "git clone error! exitcode is {}\n{}\n{}",
                output.status,
                out,
                err,
            ));
        }
        Ok((out, err))
    }
}
