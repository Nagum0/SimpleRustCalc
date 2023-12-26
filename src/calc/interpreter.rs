use super::lexer::Token;

fn eval_operation(operator: &Token, val1: i128, val2: i128) -> i128 {
    match *operator {
        Token::PLUS => val2 + val1,
        Token::MINUS => val2 - val1,
        Token::MULT => val2 * val1,
        Token::DIV => val2 / val1,
        _ => panic!("Not an operator!"),
    }
}

pub fn evaluate_expr(expr: &Vec<Token>) -> Option<i128> {
    let mut stack: Vec<i128> = Vec::new();

    for i in expr {
        match *i {
            Token::Number(n) => stack.push(n),
            _ => {
                if expr.len() < 2 {
                    return None;
                }

                let val1 = match stack.pop() {
                    Some(val) => val,
                    None => { return None; }
                };
                
                let val2 = match stack.pop() {
                    Some(val) => val,
                    None => { return None; }
                };

                stack.push(eval_operation(i, val1, val2));
            }
        }
    }
    
    let result: i128 = match stack.last() {
        Some(val) => *val,
        None => { return None; }
    };

    Some(result)
}