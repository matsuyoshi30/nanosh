use crate::Executor;
use crate::Shell;

pub struct ExportCommand {
    pub delete: bool,
    pub print: bool,
    pub args: Vec<String>,
}

impl Executor for ExportCommand {
    fn execute(&self, shell: &mut Shell) {
        if self.print {
            for (key, val) in &shell.envs {
                println!("export {}={}", key, val);
            }
        } else {
            for arg in &self.args {
                if self.delete {
                    shell.del_env(arg.to_string());
                } else {
                    let kv: Vec<&str> = arg.split("=").collect();
                    if kv.len() != 2 {
                        println!("invalid argument");
                    } else {
                        shell.set_env(kv[0].to_string(), kv[1].to_string());
                    }
                }
            }
        }
    }
}
