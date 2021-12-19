use crate::Executor;
use crate::Shell;

pub struct EchoCommand {
    pub args: Vec<String>,
}

impl Executor for EchoCommand {
    fn execute(&self, _: &mut Shell) {
        for i in &self.args {
            print!("{}", i);
            print!(" ");
        }
        println!();
    }
}
