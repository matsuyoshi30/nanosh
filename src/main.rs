use ctrlc;
use shlex;
use std::io::{stdin, stdout, Write};
use std::process;

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

        if let Some(result) = shlex::split(&input) {
            println!("{:?}", result);
        } else {
            println!("unknown shell commands");
        }
    }
}
