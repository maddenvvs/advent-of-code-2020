use super::solution::{Error, Solution};

#[derive(Clone, Copy, Debug)]
enum Token {
    OpenBracket,
    CloseBracket,
    Number(u64),
    Plus,
    Multiply,
}

impl Token {
    fn is_operation(&self) -> bool {
        use Token::*;

        matches!(self, Plus | Multiply)
    }
}

fn tokenize(expression: &str) -> Vec<Token> {
    use Token::*;

    expression
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| match ch {
            '+' => Plus,
            '*' => Multiply,
            '(' => OpenBracket,
            ')' => CloseBracket,
            ch if ch.is_digit(10) => Number(ch.to_digit(10).unwrap() as u64),
            ch => panic!("Unsupported token value: {}", ch),
        })
        .collect()
}

fn tokenize_expressions(expressions_text: &str) -> Vec<Vec<Token>> {
    expressions_text.lines().map(tokenize).collect()
}

fn eval_op(l: &Token, op: &Token, r: &Token) -> Token {
    use Token::*;

    match (l, op, r) {
        (Number(l), Plus, Number(r)) => Number(l + r),
        (Number(l), Multiply, Number(r)) => Number(l * r),
        _ => panic!("Something went wrong!"),
    }
}

fn try_swallow(stack: &mut Vec<Token>, value: Token) {
    if let Some(t) = stack.last() {
        if t.is_operation() {
            let op = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            stack.push(eval_op(&first, &op, &value));

            return;
        }
    }

    stack.push(value);
}

fn evaluate_expression(expression: &[Token]) -> Result<u64, &str> {
    use Token::*;

    let mut stack: Vec<Token> = vec![];

    for token in expression {
        match token {
            CloseBracket => {
                let res = stack.pop().unwrap();
                stack.pop();
                try_swallow(&mut stack, res);
            }
            &n @ Number(_) => try_swallow(&mut stack, n),
            &token => stack.push(token),
        }
    }

    if let Number(val) = stack[0] {
        return Ok(val);
    }

    Err("Incorrect expression")
}

fn try_swallow_advanced<T>(stack: &mut Vec<Token>, predicate: T)
where
    T: Fn(&Token) -> bool,
{
    while stack.len() > 1 && predicate(&stack[stack.len() - 2]) {
        let s = stack.pop().unwrap();
        let op = stack.pop().unwrap();
        let f = stack.pop().unwrap();

        stack.push(eval_op(&s, &op, &f));
    }
}

fn simplify(stack: &mut Vec<Token>) {
    try_swallow_advanced(stack, |t: &Token| t.is_operation())
}

fn evaluate_expression_advanced(expression: &[Token]) -> Result<u64, &str> {
    use Token::*;

    let mut stack: Vec<Token> = vec![];

    for token in expression {
        match token {
            OpenBracket => stack.push(OpenBracket),
            CloseBracket => {
                simplify(&mut stack);

                let res = stack.pop().unwrap();
                stack.pop();
                stack.push(res);
            }
            &n @ Number(_) => stack.push(n),
            Plus => {
                try_swallow_advanced(&mut stack, |t| matches!(t, Plus));
                stack.push(Plus);
            }
            Multiply => {
                simplify(&mut stack);
                stack.push(Multiply);
            }
        }
    }

    simplify(&mut stack);

    if let Number(val) = stack[0] {
        return Ok(val);
    }

    Err("Incorrect expression")
}

fn sum_of_expressions(expressions: &[Vec<Token>]) -> u64 {
    expressions
        .iter()
        .map(|e| evaluate_expression(e).unwrap())
        .sum::<u64>()
}

fn sum_of_expressions_advanced(expressions: &[Vec<Token>]) -> u64 {
    expressions
        .iter()
        .map(|e| evaluate_expression_advanced(e).unwrap())
        .sum::<u64>()
}

pub struct Day18 {}

impl Solution for Day18 {
    fn first_task(&self, expressions_text: &str) -> Result<String, Error> {
        let expressions = tokenize_expressions(expressions_text);

        Ok(sum_of_expressions(&expressions).to_string())
    }

    fn second_task(&self, expressions_text: &str) -> Result<String, Error> {
        let expressions = tokenize_expressions(expressions_text);

        Ok(sum_of_expressions_advanced(&expressions).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        let test_expressions = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71_u64),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51_u64),
            ("2 * 3 + (4 * 5)", 26_u64),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437_u64),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240_u64),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632_u64),
        ];

        for (expr, result) in &test_expressions {
            let expr = tokenize(expr);
            assert_eq!(evaluate_expression(&expr), Ok(*result));
        }
    }

    #[test]
    fn test_evaluate_expression_advanced() {
        let test_expressions = [
            ("1 + 2 * 3 + 4 * 5 + 6", 231_u64),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51_u64),
            ("2 * 3 + (4 * 5)", 46_u64),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445_u64),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060_u64),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340_u64),
        ];

        for (expr, result) in &test_expressions {
            let expr = tokenize(expr);
            assert_eq!(evaluate_expression_advanced(&expr), Ok(*result));
        }
    }
}
