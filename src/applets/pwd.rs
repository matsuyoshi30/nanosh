use crate::Executor;
use crate::Shell;

pub struct PwdCommand {}

impl Executor for PwdCommand {
    fn execute(&self, shell: &mut Shell) {
        println!("{}", shell.current_dir.to_str().unwrap());
    }
}
