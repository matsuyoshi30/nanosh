use ctrlc::set_handler;
use dirs;
use shlex::split;
use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process;

mod applets;

use crate::applets::echo::EchoCommand;
use crate::applets::export::ExportCommand;
use crate::applets::pwd::PwdCommand;

trait Executor {
    fn execute(&self, shell: &mut Shell);
}

struct Shell {
    current_dir: PathBuf,
    envs: HashMap<String, String>,
}

impl Shell {
    fn new(wd: PathBuf, envs: HashMap<String, String>) -> Self {
        Shell {
            current_dir: wd,
            envs: envs,
        }
    }

    fn set_env(&mut self, key: String, val: String) {
        self.envs.insert(key, val);
        println!("env length is {}", self.envs.len());
    }

    fn del_env(&mut self, key: String) {
        self.envs.remove(&key);
    }

    fn run(&mut self, cmd: String, args: Vec<String>) {
        match cmd.as_ref() {
            "echo" => {
                let cmd = EchoCommand { args: args };
                cmd.execute(self);
            }
            "export" => {
                let mut is_delete = false;
                let mut is_print = false;
                let mut arguments = Vec::new();
                for arg in args {
                    match arg.as_ref() {
                        "-n" => is_delete = true,
                        "-f" => is_print = true,
                        v => arguments.push(v.to_string()),
                    }
                }

                let cmd = ExportCommand {
                    delete: is_delete,
                    print: is_print,
                    args: arguments,
                };
                cmd.execute(self);
            }
            "pwd" => {
                let cmd = PwdCommand {};
                cmd.execute(self);
            }
            cmd => match process::Command::new(cmd).args(args).spawn() {
                Ok(res) => {
                    println!("{:?}", res);
                }
                Err(err) => {
                    println!("failed to execute command: {:?}", err);
                }
            },
        }
    }
}

fn main() {
    set_handler(move || {
        println!();
        println!("nanosh is finishing...");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let mut envs = HashMap::new();
    for (k, v) in env::vars() {
        envs.insert(k, v);
    }
    let cwd = dirs::home_dir().unwrap();
    let mut shell = Shell::new(cwd, envs);

    println!("nanosh start");
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("no given input");

        if let Some(mut result) = split(&input) {
            let cmd = result.remove(0);
            shell.run(cmd, result);
        } else {
            println!("unknown shell input");
        }
    }
}
