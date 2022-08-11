#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Square(Box<Expr>),
}

pub enum Operator {}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivideByZero,
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    // TODO: we should not be doing .unwrap() here, instead we should use ?
    match expr {
        Expr::Subtraction(v1, v2) => Ok(eval(v1).unwrap() - eval(v2).unwrap()),
        Expr::Addition(v1, v2) => Ok(eval(v1).unwrap() + eval(v2).unwrap()),
        Expr::Multiplication(v1, v2) => Ok(eval(v1).unwrap() * eval(v2).unwrap()),
        Expr::Division(v1, v2) => {
            println!("We are in the division method");
            let num = eval(v1).unwrap();
            println!("num: {num}");
            let den = eval(v2).unwrap();
            println!("den: {den}");
            if den == 0 {
                println!("We are gonna throw an error!");
                return Err(EvalError::DivideByZero);
            }

            Ok(num / den)
        }
        Expr::Square(v1) => Ok(eval(v1).unwrap() * eval(v1).unwrap()),
        Expr::Number(v1) => Ok(*v1),
    }
}

#[derive(Debug)]
pub enum ParseError {
    NoAvailableNumbers,
    NoWordsForThis,
}

// QUESTION FOR THE PROFESSORS! Why the hell can't we do this?!
// fn do_stuff(stack: &Vec<Expr>, action: Expr) -> Result<&Vec<Expr>, ParseError> {
//     if let (Some(first), Some(last)) = (stack.pop(), stack.pop()) {
//         stack.push(Expr::Addition(Box::new(last), Box::new(first)))
//     } else {
//         return Err(ParseError::NoAvailableNumbers);
//     };
//     return Ok(stack);
// }

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {
        match word {
            "sqr" => {
                if let Some(first) = stack.pop() {
                    stack.push(Expr::Square(Box::new(first)))
                } else {
                    return Err(ParseError::NoAvailableNumbers);
                };
            }
            _ => match word.parse::<i64>() {
                Ok(i) => stack.push(Expr::Number(i)),
                Err(_) => {
                    if let (Some(first), Some(last)) = (stack.pop(), stack.pop()) {
                        match word {
                            "+" => stack.push(Expr::Addition(Box::new(last), Box::new(first))),
                            "-" => stack.push(Expr::Subtraction(Box::new(last), Box::new(first))),
                            "*" => {
                                stack.push(Expr::Multiplication(Box::new(last), Box::new(first)))
                            }
                            "/" => stack.push(Expr::Division(Box::new(last), Box::new(first))),
                            _ => return Err(ParseError::NoWordsForThis),
                        }
                    } else {
                        return Err(ParseError::NoAvailableNumbers);
                    }
                }
            },
        }
    }
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::eval;
    use crate::parse;
    use crate::EvalError;

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn basic_input_test() {
        let input = "3 4 +";
        let expr = parse(input).unwrap();
        // Expr: [ Addition(Box::new(Number(3)), Box::new(Number(4))) ]
        let value = eval(&expr).unwrap();
        assert_eq!(value, 7);
    }

    #[test]
    fn example_division_test() {
        let input = "1 3 + 2 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn example_subtraction_test() {
        let input = "3 2 -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 1);
    }

    // TODO
    #[test]
    fn divide_zero_test() {
        let input = "3 0 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr);
        assert_eq!(value, Err(EvalError::DivideByZero));
    }
}
