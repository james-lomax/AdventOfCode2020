use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
enum Token {
    Number(i64),
    Add,
    Mul,
    SubExpr(Vec<Token>)
}

fn expr_token_chomp<'a>(tokens: &mut dyn Iterator<Item = &'a str>) -> Vec<Token> {
    let mut out = Vec::new();

    while let Some(t) = tokens.next() {
        out.push(match t {
            "+" => Token::Add,
            "*" => Token::Mul,
            "(" => Token::SubExpr(expr_token_chomp(tokens)),
            ")" => return out,
            _ => Token::Number(t.parse::<i64>()
                    .expect("Expected + - ( ) or a number!"))
        });
    }

    return out;
}

fn parse(expr: &str) -> Vec<Token> {
    let token_pat = r"([0-9]+|\+|\*|\(|\))";
    let token_pat = Regex::new(token_pat).expect("Pattern compile failed");

    let mut tokens = token_pat.find_iter(expr)
        .map(|m| m.as_str());
    
    return expr_token_chomp(&mut tokens);
}

fn eval_term(token: &Token) -> i64 {
    match token {
        Token::Number(n) => *n,
        Token::SubExpr(ex) => eval_expr(ex),
        _ => panic!("Expected term (number or sub expression)")
    }
}

fn eval_expr(tokens: &Vec<Token>) -> i64 {
    let mut acc = eval_term(tokens.first().expect("Must evaluate at least 1 token"));

    for (op, next_t) in tokens.iter().skip(1).tuples() {
        acc = match op {
            Token::Add => acc + eval_term(next_t),
            Token::Mul => acc * eval_term(next_t),
            _ => panic!("Expected operator")
        }
    }

    return acc;
}

fn eval_p1(expr: &str) -> i64 {
    let tokens = parse(expr);
    return eval_expr(&tokens);
}

// Evaluate a term, but reduce the additions first
fn reduce_add_term(token: &Token) -> i64 {
    match token {
        Token::Number(n) => *n,
        Token::SubExpr(ex) => reduce_additions(ex),
        _ => panic!("Expected term (number or sub expression)")
    }
}

// Reduce additions first and then perform normal evaluation
fn reduce_additions(tokens: &Vec<Token>) -> i64 {
    let mut out = Vec::new();
    let mut acc = reduce_add_term(tokens.first().expect("Must evaluate at least 1 token"));

    for (op, next_t) in tokens.iter().skip(1).tuples() {
        match op {
            Token::Mul => {
                out.push(Token::Number(acc));
                out.push(Token::Mul);
                acc = reduce_add_term(next_t);
            }
            Token::Add => {
                acc += reduce_add_term(next_t);
            }
            _ => panic!("Expected operator!")
        }
    }

    out.push(Token::Number(acc));

    return eval_expr(&out);
}

fn eval_p2(expr: &str) -> i64 {
    let tokens = parse(expr);
    return reduce_additions(&tokens);
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let lines: Vec<&str> = contents.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect();

    let p1: i64 = lines.iter().map(|s| eval_p1(s)).sum();
    println!("Part 1 = {}", p1);

    let p2: i64 = lines.iter().map(|s| eval_p2(s)).sum();
    println!("Part 2 = {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_p1() {
        assert_eq!(eval_p1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval_p1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval_p1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(eval_p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn test_eval_p2() {
        assert_eq!(eval_p2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(eval_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(eval_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(eval_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}
