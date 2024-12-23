mod types;
mod lexer;
mod printing;
mod parser;
mod evaluator;

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

macro_rules! parse {
    ($e1: expr, $e2: expr) => {
        match $e1.parse_program($e2) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
}

macro_rules! eval {
    ($e1: expr, $e2: expr, $e3: expr) => {
        match $e1.eval_program($e2, $e3) {
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
    Ast,
    Group,
    Eval
}

#[derive(Parser, Debug)]
#[command(version, about = "Lambda calculus evaluator", long_about = None)]
struct Args {
    #[arg(value_enum, long("prog"), default_value_t=Program::Eval, help="Select a part of the program to run")]
    program: Program,

    #[arg(long, help="Use rightmost associativity for binary operators")]
    right: bool,

    #[arg(long("no-prec"), help="Disable precedence rules for binary operators")]
    noprec: bool,

    #[arg(long, help="Use dynamic scoping rules in the evaluator")]
    dynamic: bool,

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
    // Create parser
    let mut parse = parser::Parser::new(args.noprec, args.right);
    // Create evaluator
    let mut evaluate = evaluator::Evaluator::new();

    // Run lexer program
    match args.program{
        Program::Lex => {
            // Generate lexer output
            let lexer_out = lex!(lex, &input);
            // Print token stream
            printing::print_token_stream(&lexer_out)
        },
        // Run parser program
        Program::Ast => {
            // Generate lexer output
            let lexer_out = lex!(lex, &input);
            // Generate parser output
            let parser_out = parse!(parse, lexer_out);
            // Print abstract syntax tree
            printing::print_program(&parser_out);
        },
        // Run grouping program
        Program::Group => {
            // Generate lexer output
            let lexer_out = lex!(lex, &input);
            // Generate parser output
            let parser_out = parse!(parse, lexer_out);
            // Print grouping
            printing::print_group(&parser_out);
        },
        // Run evaluator program
        Program::Eval => {
            // Generate lexer output
            let lexer_out = lex!(lex, &input);
            // Generate parser output
            let parser_out = parse!(parse, lexer_out);
            // Evaluate parser output
            let eval_out = eval!(evaluate, parser_out, args.dynamic);
            // Print grouping
            printing::print_grouped_expression(&eval_out, false);
            // Print newline
            println!()
        }
    }
}