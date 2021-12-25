use ctrlc::set_handler;
use dirs;
use shlex::split;
use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process;

mod applets;

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
            "cd" => {
                let cmd = applets::cd::CdCommand { arg: &args[0] };
                cmd.execute(self);
            }
            "echo" => {
                let cmd = applets::echo::EchoCommand { args: args };
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

                let cmd = applets::export::ExportCommand {
                    delete: is_delete,
                    print: is_print,
                    args: arguments,
                };
                cmd.execute(self);
            }
            "ls" => {
                let mut is_all = false;
                for arg in args {
                    match arg.as_ref() {
                        "-a" => is_all = true,
                        _ => {} // TODO handle arguments
                    }
                }
                let cmd = applets::ls::LsCommand { all: is_all };
                cmd.execute(self);
            }
            "pwd" => {
                let cmd = applets::pwd::PwdCommand {};
                cmd.execute(self);
            }
            cmd => match process::Command::new(cmd).args(args).spawn() {
                Ok(mut child) => {
                    child.wait().expect("failed to execute child process");
                }
                Err(err) => {
                    println!("failed to execute {:?}: {:?}", cmd, err);
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
            if result.len() == 0 {
                continue;
            }
            let cmd = result.remove(0);
            shell.run(cmd, result);
        } else {
            println!("unknown shell input");
        }
    }
}
