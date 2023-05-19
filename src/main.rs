#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::style,
    clippy::complexity
)]
//Disable later
#![allow(clippy::missing_const_for_fn)]

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

use crate::lexer::Lexer;
use crate::parser::Parser;

mod expressions;
mod lexer;
mod parser;
mod token;
mod utils;

fn run(src: &str) -> Result<(), &str> {
    println!("RUNNING: {src}");
    let mut lex = Lexer::new(src.to_string());

    lex.scan_tokens();
    lex.print_tokens();

    let tokens_source = lex.transfer_tokens_src();

    let tokens = tokens_source.tokens;

    // Move tokens and source from lexer into parser
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    println!("\n\n ============== \n{expression:#?}");

    Ok(())
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path);
    let contents = contents.map_or_else(
        |_| {
            println!("Error al abrir el archivo {path}");
            exit(1);
        },
        |e| e,
    );
    match run(&contents) {
        Ok(_) => exit(0),
        Err(_) => todo!(),
    }
}

fn run_repl() {
    loop {
        let mut input = String::new();
        print!("EspLox> ");
        io::stdout()
            .flush()
            .expect("Error a la hora de \"flushear\" stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Error a la hora de leer desde stdin");

        if input.is_empty() || input == "salir" {
            break;
        }

        let result = run(&input);
        if let Err(err) = result {
            utils::error(1, err);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        println!("Uso: {} [archivo.esp]", args[0]);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}
