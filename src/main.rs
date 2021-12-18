use ctrlc;
use shlex;
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
            print!("{}", i);
            print!(" ");
        }
        println!();
    }
}

fn main() {
    ctrlc::set_handler(move || {
        println!();
        println!("nanosh is finishing...");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    println!("nanosh start");
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("no given input");

        if let Some(mut result) = shlex::split(&input) {
            let cmd = result.remove(0);
            match cmd.as_ref() {
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
                }
            }
        } else {
            println!("unknown shell input");
        }
    }
}
