use project05_assembler::parser::Parser;
use std::env;
use std::process;

fn main() {
    // $ HackAssembler Add.asm
    let args: Vec<String> = env::args().collect();
    // Check filename is provided
    if args.len() < 2 {
        eprintln!("No files provided!");
        process::exit(1);
    }
    // Filename
    let filename = &args[1];
    // Create a new Parser
    let parser = Parser::new(filename);
    println!("{:#?}", parser.input)
}
