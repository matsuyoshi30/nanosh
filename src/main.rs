use ctrlc::set_handler;
use nix::{
    sys::wait::waitpid,
    unistd::{fork, write, ForkResult},
};
use shlex::split;
use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process;

trait Executor {
    fn execute(&self);
}

struct EchoCommand {
    args: Vec<String>,
}

impl Executor for EchoCommand {
    fn execute(&self) {
        for i in &self.args {
            write(libc::STDOUT_FILENO, i.as_bytes()).ok();
            write(libc::STDOUT_FILENO, " ".as_bytes()).ok();
        }
        write(libc::STDOUT_FILENO, "\n".as_bytes()).ok();
        unsafe { libc::_exit(0) };
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

    println!("nanosh start");
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("no given input");

        if let Some(mut result) = split(&input) {
            let cmd = result.remove(0);
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    waitpid(child, None).unwrap();
                }
                Ok(ForkResult::Child) => match cmd.as_ref() {
                    "echo" => {
                        let mut arguments = Vec::new();
                        for i in result {
                            arguments.push(i);
                        }
                        let cmd = EchoCommand { args: arguments };
                        cmd.execute();
                    }
                    _ => {
                        println!("unknown command");
                        unsafe { libc::_exit(0) };
                    }
                },
                Err(_) => println!("process fork failed"),
            }
        } else {
            println!("unknown shell input");
        }
    }
}
