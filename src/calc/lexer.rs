//! |--- Tokenize string expression ---|

use colored::Colorize;

#[derive(Clone, Copy)]
pub enum Token {
    Number(i128),
    PLUS,
    MINUS,
    MULT,
    DIV,
}

#[allow(unused)]
impl Token {
    // Parse string input into tokens:
    pub fn tokenize(expr: String) -> Option<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: String = String::new();

        for i in expr.chars() {
            if i.is_whitespace() {
                continue;
            }
            else if i.is_numeric() {
                buffer.push(i);
            }
            else {
                let n: i128 = match buffer.parse::<i128>() {
                    Ok(val) => val,
                    Err(_) => { return None; }
                };
                tokens.push(Self::Number(n));

                match i {
                    '+' => tokens.push(Self::PLUS),
                    '-' => tokens.push(Self::MINUS),
                    '*' => tokens.push(Self::MULT),
                    '/' => tokens.push(Self::DIV),
                    _ => {
                        println!("{}::Unkown token: {}", "Error".red(), i);
                        return None;
                    }
                }

                buffer.clear();
            }
        }

        if !buffer.is_empty() {
            let n: i128 = match buffer.parse::<i128>() {
                Ok(val) => val,
                Err(_) => { return None; }
            };
            tokens.push(Self::Number(n));
        }

        Some(tokens)
    }

    // Print token:
    pub fn print(&self) {
        match self {
            Self::PLUS => println!("Token: +"),
            Self::MINUS => println!("Token: -"),
            Self::MULT => println!("Token: *"),
            Self::DIV => println!("Token: /"),
            Self::Number(n) => println!("Token: Number({})", n),
        }
    }

    // Print tokens:
    pub fn print_tokens(tokens: &Vec<Self>) {
        for i in tokens {
            i.print();
        }
    }
}