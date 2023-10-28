use std::env;
use std::fs;
use std::io;
use std::process;

use scanner::Scanner;

static mut HAD_ERROR: bool = false;
mod token_type;
mod token;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]")
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt()
    }

    println!("{}", args.len());
}

fn run_file(path: String) {
    let file = fs::read_to_string(path).expect("Should have been able to read the file");
    run(file);

    if unsafe { HAD_ERROR } { process::exit(65) };
}

fn run_prompt() {
    
    loop {
        print!("> ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        run(line);
        unsafe {HAD_ERROR = false;}
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for  token in tokens.iter() {
        println!("{}", token.to_string());
    }
    
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, _where: &str, message: &str) {
    println!("[line  ${line}] Error ${_where} : ${message}" ,);
    unsafe { HAD_ERROR = true };

}