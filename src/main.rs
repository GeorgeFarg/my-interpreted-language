// main.rs

mod lexer;
mod parser;
mod runtime;

// use lexer::{Token, tokenize};
use parser::Parser;
// use std::{env::args, fs::read_to_string};
use std::io::stdin;

use runtime::{RuntimeValue, evaluate};

use crate::runtime::{
    Environment,
    values::{BoolVal, NumberVal, ValueType},
};

fn main() {
    // let arguments: Vec<String> = args().collect();
    // if arguments.len() < 2 {
    //     eprintln!("Usage: Cargo run <file_path>");
    //     std::process::exit(1);
    // }
    // let file_path = &arguments[1];
    // let source_code = read_to_string(file_path).expect("Failed to read file");

    let mut parser = Parser::new();
    let mut input = String::new();
    let mut env = Environment::new();
    env.declare_var(
        "x",
        RuntimeValue::NumberVal(NumberVal::new(ValueType::Number, 100.0)),
    );
    env.declare_var("true", RuntimeValue::BooleanVal(BoolVal::new(true)));
    env.declare_var("false", RuntimeValue::BooleanVal(BoolVal::new(false)));
    loop {
        println!("Enter Valid code:");
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            println!("Exited using command /EXIT");
            break;
        }

        let ast = parser.produce_ast(input.trim());

        println!("The AST: {:#?}", ast);

        let result: RuntimeValue = evaluate(ast, &mut env);
        println!("Result: {:#?}", result);
    }
}
