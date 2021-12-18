use std::io::{stdin, stdout, Write};

fn main() {
    println!("nanosh start");
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("no given input");
        println!("{}", input);
    }
}
