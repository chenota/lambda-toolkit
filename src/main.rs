mod types;
mod lexer;
mod printing;

use clap::Parser;
use std::io;
use std::fs;
use std::process;

macro_rules! lex {
    ($e1: expr, $e2: expr) => {
        match $e1.generate($e2) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Lambda calculus evaluator", long_about = None)]
struct Args {
    #[arg(long, group="program", help="Run in lexer mode.")]
    lex: bool,

    #[arg(long, group="program", help="Run in parse mode.")]
    parse: bool,

    #[arg(long, group="program", help="Run in script mode. Default mode if no run flags are set.")]
    script: bool,

    #[arg(help="Path to program file. Use stdin if not specified.")]
    fname: Option<String>
}

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Get program input
    let input: String = match args.fname {
        Some(p) => {
            match fs::read_to_string(p) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("file error"); 
                    process::exit(1)
                }
            }
        },
        None => {
            match io::read_to_string(io::stdin()) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("stdin error"); 
                    process::exit(1)
                }
            }
        }
    };

    // Create lexer
    let mut lex = lexer::Lexer::new();

    // Run lexer program
    if args.lex {
        // Generate lexer output
        let lexer_out = lex!(lex, &input);
        // Print token stream
        printing::print_token_stream(&lexer_out)
    } 
    // Run parser program
    else if args.parse {

    } 
    // Run evaluator program
    else {

    }
}