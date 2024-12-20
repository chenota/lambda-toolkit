mod types;
mod lexer;
mod printing;
mod parser;

use clap::Parser;
use std::io;
use std::fs;
use std::process;

// Macros

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

// Argument parser

#[derive(clap::ValueEnum, Clone, Debug)]
enum Program {
    Lex,
    Parse,
    Script
}

#[derive(Parser, Debug)]
#[command(version, about = "Lambda calculus evaluator", long_about = None)]
struct Args {
    #[arg(value_enum, long, default_value_t=Program::Script, help="Select a part of the program to run.")]
    program: Program,

    #[arg(help="Optional path to program file. Use stdin if not specified.")]
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
    match args.program{
        Program::Lex => {
            // Generate lexer output
            let lexer_out = lex!(lex, &input);
            // Print token stream
            printing::print_token_stream(&lexer_out)
        },
        // Run parser program
        Program::Parse => {

        },
        // Run evaluator program
        Program::Script => {

        }
    }
}