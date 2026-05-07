use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Guna: {} <fail.nl>", args[0]);
        process::exit(1);
    }
    let source = fs::read_to_string(&args[1]).expect("Gagal membaca fail sumber");
    // 1. Lexer
    let mut lexer = nl_lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Ralat lekser: {}", e);
            process::exit(1);
        }
    };
    // 2. Parser
    let mut parser = nl_parser::Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Ralat sintaks: {}", e);
            process::exit(1);
        }
    };
    // 3. Semantik
    if let Err(errors) = nl_semantic::resolve(&ast) {
        eprintln!("Ralat semantik:");
        for err in errors {
            eprintln!("  {}", err);
        }
        process::exit(1);
    }
    // 4. Penjanaan kod C ke stdout
    if let Err(e) = nl_codegen::generate(&ast, &mut std::io::stdout()) {
        eprintln!("Ralat penjanaan kod: {}", e);
        process::exit(1);
    }
}
