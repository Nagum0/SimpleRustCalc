use colored::ColoredString;
use colored::Colorize;

use std::io;
use std::io::Write;

mod calc;
use calc::lexer::Token;
use calc::parser;
use calc::interpreter;

fn main() {
    let err_header: ColoredString = "Error".red();

    loop {
        let mut buff: String = String::new();
        print!("calc> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut buff) {
            Ok(_) => { buff = buff.trim_end().to_string(); },
            Err(_) => {
                println!("{}::Error while reading input...", err_header);
                continue;
            },
        }

        // EXIT:
        if buff == ":q" || buff == "quit"{
            break;
        }

        let tokens: Vec<Token> = match Token::tokenize(buff) {
            Some(vec) => vec,
            None => {
                println!("{}::Error while tokenizing input...", err_header);
                continue;
            }
        };

        match parser::parse_to_postfix(&tokens) {
            Ok(expr) => {
                let result = match interpreter::evaluate_expr(&expr) {
                    Some(val) => val,
                    None => {
                        println!("{}::Error while calculating expression...", err_header);
                        continue;
                    }
                };

                println!("{}", result);
            },
            Err(_) => {
                println!("{}::Error while parsing input...", err_header);
                continue;
            }
        }
    }
}