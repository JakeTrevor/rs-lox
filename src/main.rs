mod parse;

use std::io::{Write, stdin, stdout};
use std::{env, fs, process};

use crate::parse::lex::scan::Scanner;
use crate::parse::parse::parser::Parser;

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(1);
    }

    if args.len() == 2 {
        run_file(
            args.nth(1)
                .expect("There to be a second argument when args.len() = 2"),
        )
    } else {
        run_prompt()
    }
}

fn run_prompt() -> std::io::Result<()> {
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();

        match stdin().read_line(&mut s) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        run(s, "".to_owned())?
    }
}

fn run_file(filename: String) -> std::io::Result<()> {
    let contents = fs::read_to_string(&filename).expect("Should have been able to read the file");
    run(contents, filename)
}

fn run(source: String, filename: String) -> std::io::Result<()> {
    let (tokens, errs) = Scanner::new(&filename, &source).scan();

    if errs.len() > 0 {
        for e in errs.iter() {
            e.print(source.to_owned())?
        }
        return Ok(());
    }

    let result = Parser::new(tokens, filename).parse();

    match result {
        Ok(ast) => println!("{}", ast),
        Err(err) => err.print(source.to_owned())?,
    }

    return Ok(());
}
