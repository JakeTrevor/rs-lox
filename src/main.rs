use std::io::{Write, stdin, stdout};
use std::{env, fs, process};

use crate::lex::Scanner;

mod lex;

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(1);
    }

    if args.len() == 2 {
        run_file(
            args.nth(1)
                .expect("There to be a second argument when args.len() = 2"),
        );
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();

        match stdin().read_line(&mut s) {
            Err(_) => break,
            Ok(_) => {}
        }

        run(s);
    }
}

fn run_file(filename: String) {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    //  ^? contents : String
    run(contents)
}

fn run(source: String) {
    let (tokens, errs) = Scanner::new(&source).scan();

    // For now, just print the tokens.
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}
