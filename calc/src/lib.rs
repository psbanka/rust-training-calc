    #[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Sqr(Box<Expr>),
    BinOp {
        op_kind: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOpKind {
    Div,
    Add,
    Sub,
    Mult,
}

#[derive(Debug)]
pub enum EvalError {
    DivideByZero
}
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken,
    ExtraInput,
    NotEnoughOperands
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack = Vec::new();
    let words = input.split_ascii_whitespace();
    for w in words {
        match w {
            "+" | "-" | "/" | "*" => {
                let left = stack.pop();
                let right = stack.pop();
                if let (Some(left), Some(right)) = (left, right) {
                    match w {
                        "+" => stack.push(Expr::BinOp { op_kind: BinOpKind::Add, left: Box::new(left), right: Box::new(right) }),
                        "-" => stack.push(Expr::BinOp { op_kind: BinOpKind::Sub, left: Box::new(left), right: Box::new(right) }),
                        "/" => stack.push(Expr::BinOp { op_kind: BinOpKind::Div, left: Box::new(left), right: Box::new(right) }),
                        "*" => stack.push(Expr::BinOp { op_kind: BinOpKind::Mult, left:Box::new( left), right: Box::new(right) }),
                        _ => unreachable!()
                    }
                } else {
                    return Err(ParseError::NotEnoughOperands)
                }
            },
            "sqr" => {
                let base = stack.pop();
                match base {
                    Some(base) => stack.push(Expr::Sqr(Box::new(base))),
                    _ => return Err(ParseError::NotEnoughOperands)
                }
            },
            n => match n.parse() {
                Ok(n) => stack.push(Expr::Number(n)),
                Err(e) => return Err(ParseError::UnexpectedToken),
            },
        }
    }
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}

pub fn eval(expr: &Expr) -> Result<i32, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::BinOp { op_kind, left, right } => {
            let left_res = eval(left)?;
            let right_res = eval(right)?;
            match op_kind {
                BinOpKind::Div => {
                    if right_res == 0 {
                        Err(EvalError::DivideByZero)
                    } else {
                        Ok(left_res / right_res)
                    }
                },
                BinOpKind::Add => Ok(left_res + right_res),
                BinOpKind::Sub => Ok(left_res - right_res),
                BinOpKind::Mult => Ok(left_res * right_res),
            }
        },
        Expr::Sqr(base) => {
            let base_res = eval(base)?;
            Ok(base_res.pow(2))
        },
    }
}

#[test]
fn smoke_test() {
    let input = "3 sqr 4 sqr + 5 sqr -";
    let expr = parse(input).unwrap();
    assert_eq!(eval(&expr).unwrap(), 0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
