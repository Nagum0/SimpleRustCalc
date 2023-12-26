//! |--- Parse infix expression to postfix ---|

//use colored::Colorize;
use super::lexer::Token;

fn has_higher_precedence(stack_top: &Token, operator: &Token) -> bool {
    match (stack_top, operator) {
        (Token::PLUS, Token::MULT) => true,
        (Token::MINUS, Token::MULT) => true,
        (Token::PLUS, Token::DIV) => true,
        (Token::MINUS, Token::DIV) => true,
        _ => false,
    }
}

pub fn parse_to_postfix(expr: &Vec<Token>) -> Result<Vec<Token>, &str> {
    let mut postfix: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for i in expr {
        match *i {
            Token::Number(n) => postfix.push(Token::Number(n)),
            _ => {
                if stack.is_empty() {
                    stack.push(*i);
                }
                else if has_higher_precedence(&stack.last().unwrap(), i) {
                    stack.push(*i);
                }
                else {
                    while !stack.is_empty() && !has_higher_precedence(&stack.last().unwrap(), i) {
                        postfix.push(stack.pop().unwrap());
                    }
                    stack.push(*i);
                }
            }
        }
    }

    // Push remaining operators from the stack to the postfix expression:
    while let Some(op) = stack.pop() {
        postfix.push(op);
    }

    Ok(postfix)
}