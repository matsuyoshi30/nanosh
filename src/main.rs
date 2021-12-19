use ctrlc::set_handler;
use shlex::split;
use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process;

struct Shell {
    envs: HashMap<String, String>,
}

impl Shell {
    fn new(envs: HashMap<String, String>) -> Self {
        Shell { envs: envs }
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

trait Executor {
    fn execute(&self, shell: &mut Shell);
}

struct EchoCommand {
    args: Vec<String>,
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

struct ExportCommand {
    delete: bool,
    print: bool,
    args: Vec<String>,
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
    let mut shell = Shell::new(envs);

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
