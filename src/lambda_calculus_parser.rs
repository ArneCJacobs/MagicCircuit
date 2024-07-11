use std::{
    collections::VecDeque,
    sync::{Mutex, OnceLock},
};

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::{tag, take_while, take_while1, take_while_m_n}, character::complete::space0, combinator::{all_consuming, map_res, opt}, multi::{many0, many1}, sequence::{delimited, preceded, separated_pair, Tuple}, IResult, Parser
};

fn indent_reset() {
    let mut indent = indent().lock().unwrap();
    *indent = 0;
}

fn indent() -> &'static Mutex<u8> {
    static INDENT: OnceLock<Mutex<u8>> = OnceLock::new();
    INDENT.get_or_init(|| Mutex::new(0))
}

fn get_indent() -> u8 {
    let indent = indent().lock().unwrap();
    *indent
}

struct ScopeCall<F: FnMut()> {
    c: F,
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

macro_rules! defer {
    ($e:expr) => {
        let _scope_call = ScopeCall {
            c: || -> () {
                $e;
            },
        };
    };
}

fn inc_indent() {
    let mut indent = indent().lock().unwrap();
    *indent += 1;
}

fn dec_indent() {
    let mut indent = indent().lock().unwrap();
    *indent -= 1;
}

fn println_with_indent(s: &str) {
    let indent = get_indent();
    if indent == 0 {
        println!("{}", s);
        return;
    }
    // for each indent, print a pipe
    print!("{}", "|".repeat(indent as usize));
    println!("{}", s);
}

#[derive(Debug, PartialEq)]
enum LambdaExpression {
    Variable(String),
    Abstraction(String, Box<LambdaExpression>),
    Application(Box<LambdaExpression>, Box<LambdaExpression>),
}

fn parse_string_variable(expr: &str) -> IResult<&str, &str> {
    inc_indent();
    defer!(dec_indent());
    println_with_indent("parsing_variable");
    println_with_indent(&format!("here0: {:?}", expr));
    let (input, variable) = take_while_m_n(1, 1, |c: char| c.is_alphabetic())(expr)?;
    println_with_indent(&format!("here1: {:?}", input));

    Ok((input, variable))
}

fn parse_lambda_variable(expr: &str) -> IResult<&str, LambdaExpression> {
    inc_indent();
    defer!(dec_indent());
    let (input, variable) = parse_string_variable(expr)?;

    Ok((input, LambdaExpression::Variable(variable.to_string())))
}

fn parse_lambda_abstraction(expr: &str) -> IResult<&str, LambdaExpression> {
    inc_indent();
    defer!(dec_indent());
    println_with_indent("parsing_abstraction");
    println_with_indent(&format!("here0: {:?}", expr));
    let (input, _) = tag("λ")(expr)?;
    println_with_indent(&format!("here1: {:?}", input));
    let (input, LambdaExpression::Variable(variable)) = parse_lambda_variable(input)? else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    };
    println_with_indent(&format!("here2: {:?}", input));
    let (input, _) = tag(".")(input)?;
    println_with_indent(&format!("here3: {:?}", input));
    let (input, expr) = parse_lambda_expression(input)?;
    println_with_indent(&format!("here4: {:?}", input));
    Ok((
        input,
        LambdaExpression::Abstraction(variable.to_string(), Box::new(expr)),
    ))
}

fn parse_string_balanced_expression(expr: &str) -> IResult<&str, &str> {
    inc_indent();
    defer!(dec_indent());
    println_with_indent("parsing_balanced_expression");
    println_with_indent(&format!("here0: {:?}", expr));
    if expr.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            expr,
            nom::error::ErrorKind::IsNot,
        )));
    }
    let mut count = 0;

    for (index, ch) in expr.char_indices() {
        match ch {
            '(' => count += 1,
            ')' => {
                if count == 0 {
                    println_with_indent("more closed brackets then open ones");
                    // print currrent char and index
                    //println_with_indent(&format!("index: {:?}, char: {:?}", index, ch));
                    println_with_indent(&format!("parsed so far: {:?}, rest: {:?}", &expr[..index], &expr[index..]));
                    //println_with_indent(&format!("parsed so far: {:?}, rest: {:?}", &chars[..index], &chars[index..]));
                    return Ok((&expr[index..], &expr[..index]));
                }

                count -= 1;
            }
            _ => (),
        }
    }
    println_with_indent(&format!("count: {:?}", count));
    if count != 0 {
        println_with_indent(&format!("expression not balanced: {:?}", expr));
        return Err(nom::Err::Error(nom::error::Error::new(
            expr,
            nom::error::ErrorKind::IsNot,
        )));
    }
    println_with_indent(&format!("here1: {:?}", expr));

    Ok(("", expr))
}

fn parse_lambda_application(expr: &str) -> IResult<&str, LambdaExpression> {
    inc_indent();
    //if *indent().lock().unwrap() > 20 {
    //    return Err(nom::Err::Error(nom::error::Error::new(
    //        expr,
    //        nom::error::ErrorKind::IsNot,
    //    )));
    //}
    defer!(dec_indent());
    println_with_indent("parsing_application");
    println_with_indent(&format!("here0: {:?}", expr));
    //let parser = alt((
    //    parse_string_variable,
    //    parse_string_balanced_expression,
    //));
    //println_with_indent(&format!("here0: {:?}", many1(parser).parse(expr)));

    let parser = preceded(
        space0, 
        alt((
            parse_string_variable,
            delimited(tag("("), parse_string_balanced_expression, tag(")")),
        ))
    );
    let (input, res) = all_consuming(many1(parser)).parse(expr)?;
    println_with_indent(&format!("here1: {:?}", input));
    println_with_indent(&format!("here1 res: {:?}", res));
    if res.len() < 2 {
        println_with_indent("not enough expressions");
        return Err(nom::Err::Error(nom::error::Error::new(
            expr,
            nom::error::ErrorKind::IsNot,
        )));
    }
    println_with_indent(&format!("here1.5: {:?}", res));
    let parsed_expressions = res
        .iter()
        .map(|s| 
            //all_consuming(
                alt((
                    all_consuming(parse_lambda_variable),
                    all_consuming(parse_lambda_expression),
                ))
            //)
            .parse(s)
        )
        .collect::<Result<Vec<_>, _>>()?;
    println_with_indent(&format!("here2: {:?}", parsed_expressions));
    let mut expressions = parsed_expressions
        .into_iter()
        .map(|(_, expr)| expr)
        .collect::<VecDeque<_>>();

    println_with_indent(&format!("here3: {:?}", expressions));
    while expressions.len() > 1 {
        let expr1 = expressions.pop_front().unwrap();
        let expr2 = expressions.pop_front().unwrap();
        expressions.push_front(LambdaExpression::Application(
            Box::new(expr1),
            Box::new(expr2),
        ));
    }
    println_with_indent(&format!("here4: {:?}", expressions));

    Ok((input, expressions.pop_front().unwrap()))
}

fn parse_lambda_expression(expr: &str) -> IResult<&str, LambdaExpression> {
    inc_indent();
    defer!(dec_indent());
    println_with_indent("parsing_expression");
    println_with_indent(&format!("here0: {:?}", expr));
    alt((
        parse_lambda_abstraction,
        parse_lambda_application,
        parse_lambda_variable,
    ))
    .parse(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_lambda_expression {
        ($name:ident, $expr:expr, $expected:expr) => {
            ::paste::paste! {
                #[test]
                fn [<test_parse_lambda_expression_$name>]() {
                    indent_reset();
                    let expr = String::from($expr);
                    let parsed_expr = parse_lambda_expression(&expr);
                    assert_eq!(parsed_expr,
                        Ok(
                            (
                                "",
                                $expected
                            )
                        )
                    );
                }
            }
        };
    }

    #[test]
    fn test_parse_lambda_application_simple() {
        indent_reset();
        let expr = String::from("x y");
        let parsed_expr = parse_lambda_application(&expr);
        assert!(parsed_expr.is_ok());
        let (input, expr) = parsed_expr.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            expr,
            LambdaExpression::Application(
                Box::new(LambdaExpression::Variable("x".to_string())),
                Box::new(LambdaExpression::Variable("y".to_string())),
            )
        );
    }

    #[test]
    fn test_parse_lambda_variable() {
        indent_reset();
        let expr = String::from("x");
        let parsed_expr = parse_lambda_variable(&expr);
        assert!(parsed_expr.is_ok());
        let (input, expr) = parsed_expr.unwrap();
        assert_eq!(input, "");
        assert_eq!(expr, LambdaExpression::Variable("x".to_string()));
    }

    #[test]
    fn test_parse_lambda_abstraction() {
        indent_reset();
        let expr = String::from("λx.x");
        let parsed_expr = parse_lambda_abstraction(&expr);
        assert!(parsed_expr.is_ok());
        let (input, expr) = parsed_expr.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            expr,
            LambdaExpression::Abstraction(
                "x".to_string(),
                Box::new(LambdaExpression::Variable("x".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_lambda_application_complex_1() {
        indent_reset();
        let expr = String::from("(x y)z");
        let parsed_expr = parse_lambda_application(&expr);
        assert_eq!(
            parsed_expr,
            Ok((
                "",
                LambdaExpression::Application(
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("x".to_string())),
                        Box::new(LambdaExpression::Variable("y".to_string())),
                    )),
                    Box::new(LambdaExpression::Variable("z".to_string())),
                )
            ))
        );
    }

    #[test]
    fn test_parse_lambda_application_complex_2() {
        indent_reset();
        let expr = String::from("(x y)(a b)");
        let parsed_expr = parse_lambda_application(&expr);
        assert_eq!(
            parsed_expr,
            Ok((
                "",
                LambdaExpression::Application(
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("x".to_string())),
                        Box::new(LambdaExpression::Variable("y".to_string())),
                    )),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("a".to_string())),
                        Box::new(LambdaExpression::Variable("b".to_string())),
                    )),
                )
            ))
        );
    }

    test_parse_lambda_expression!(
        true,
        "λx.λy.x",
        LambdaExpression::Abstraction(
            "x".to_string(),
            Box::new(LambdaExpression::Abstraction(
                "y".to_string(),
                Box::new(LambdaExpression::Variable("x".to_string()))
            ))
        )
    );

    test_parse_lambda_expression!(
        false,
        "λx.λy.y",
        LambdaExpression::Abstraction(
            "x".to_string(),
            Box::new(LambdaExpression::Abstraction(
                "y".to_string(),
                Box::new(LambdaExpression::Variable("y".to_string()))
            ))
        )
    );

    test_parse_lambda_expression!(
        s,
        "λx.λy.λz.(x z)(y z)",
        LambdaExpression::Abstraction(
            "x".to_string(),
            Box::new(LambdaExpression::Abstraction(
                "y".to_string(),
                Box::new(LambdaExpression::Abstraction(
                    "z".to_string(),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Application(
                            Box::new(LambdaExpression::Variable("x".to_string())),
                            Box::new(LambdaExpression::Variable("z".to_string())),
                        )),
                        Box::new(LambdaExpression::Application(
                            Box::new(LambdaExpression::Variable("y".to_string())),
                            Box::new(LambdaExpression::Variable("z".to_string())),
                        )),
                    ))
                ))
            ))
        )
    );

    test_parse_lambda_expression!(
        optional_space_application_1,
        "f (x x)",
        LambdaExpression::Application(
            Box::new(LambdaExpression::Variable("f".to_string())),
            Box::new(LambdaExpression::Application(
                Box::new(LambdaExpression::Variable("x".to_string())),
                Box::new(LambdaExpression::Variable("x".to_string())),
            ))
        )
    );

    test_parse_lambda_expression!(
        optional_space_application_2,
        "f(x x)",
        LambdaExpression::Application(
            Box::new(LambdaExpression::Variable("f".to_string())),
            Box::new(LambdaExpression::Application(
                Box::new(LambdaExpression::Variable("x".to_string())),
                Box::new(LambdaExpression::Variable("x".to_string())),
            ))
        )
    );

    #[test]
    fn test_parse_balanced_expression() {
        indent_reset();
        let expr = String::from("λx.f (x x))");
        let (rest, parsed) = parse_string_balanced_expression(&expr).unwrap();
        assert_eq!(rest, ")");
        assert_eq!(parsed, "λx.f (x x)");
    }

    #[test]
    fn test_parse_balanced_expression_2() {
        indent_reset();
        let expr = String::from("x x)");
        let (rest, parsed) = parse_string_balanced_expression(&expr).unwrap();
        assert_eq!(rest, ")");
        assert_eq!(parsed, "x x");
    }

    test_parse_lambda_expression!(
        y_part,
        "(λx.x x)(λx.f (x x))",
        LambdaExpression::Application(
            Box::new(LambdaExpression::Abstraction(
                "x".to_string(),
                Box::new(LambdaExpression::Application(
                    Box::new(LambdaExpression::Variable("x".to_string())),
                    Box::new(LambdaExpression::Variable("x".to_string())),
                ))
            )),
            Box::new(LambdaExpression::Abstraction(
                "x".to_string(),
                Box::new(LambdaExpression::Application(
                    Box::new(LambdaExpression::Variable("f".to_string())),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("x".to_string())),
                        Box::new(LambdaExpression::Variable("x".to_string())),
                    )),
                ))
            ))
        )
    );

    test_parse_lambda_expression!(
        y,
        "λf.(λx.x x)(λx.f (x x))",
        LambdaExpression::Abstraction(
            "f".to_string(),
            Box::new(LambdaExpression::Application(
                Box::new(LambdaExpression::Abstraction(
                    "x".to_string(),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("x".to_string())),
                        Box::new(LambdaExpression::Variable("x".to_string())),
                    ))
                )),
                Box::new(LambdaExpression::Abstraction(
                    "x".to_string(),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("f".to_string())),
                        Box::new(LambdaExpression::Application(
                            Box::new(LambdaExpression::Variable("x".to_string())),
                            Box::new(LambdaExpression::Variable("x".to_string())),
                        )),
                    ))
                ))
            ))
        )
    );

    test_parse_lambda_expression!(
        church_numeral_4,
        "λf.λx.f(f(f(f x)))",
        LambdaExpression::Abstraction(
            "f".to_string(),
            Box::new(LambdaExpression::Abstraction(
                "x".to_string(),
                Box::new(LambdaExpression::Application(
                    Box::new(LambdaExpression::Variable("f".to_string())),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("f".to_string())),
                        Box::new(LambdaExpression::Application(
                            Box::new(LambdaExpression::Variable("f".to_string())),
                            Box::new(LambdaExpression::Application(
                                Box::new(LambdaExpression::Variable("f".to_string())),
                                Box::new(LambdaExpression::Variable("x".to_string())),
                            )),
                        )),
                    )),
                )),
            )),
        )
    );

    test_parse_lambda_expression!(
        left_associative,
        "a b c",
        LambdaExpression::Application(
            Box::new(LambdaExpression::Application(
                Box::new(LambdaExpression::Variable("a".to_string())),
                Box::new(LambdaExpression::Variable("b".to_string())),
            )),
            Box::new(LambdaExpression::Variable("c".to_string())),
        )
    );

    //λn.λf.λx.
    // n(λg.λh.h(g f))(λu.x)(λu.u)
    test_parse_lambda_expression!(
        pred,
        "λn.λf.λx.n(λg.λh.h(g f))(λu.x)(λu.u)",
        LambdaExpression::Abstraction(
            "n".to_string(),
            Box::new(LambdaExpression::Abstraction(
                "f".to_string(),
                Box::new(LambdaExpression::Abstraction(
                    "x".to_string(),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Application(
                            Box::new(LambdaExpression::Application(
                                Box::new(LambdaExpression::Variable("n".to_string())),
                                Box::new(LambdaExpression::Abstraction(
                                    "g".to_string(),
                                    Box::new(LambdaExpression::Abstraction(
                                        "h".to_string(),
                                        Box::new(LambdaExpression::Application(
                                            Box::new(LambdaExpression::Variable("h".to_string())),
                                            Box::new(LambdaExpression::Application(
                                                Box::new(LambdaExpression::Variable("g".to_string())),
                                                Box::new(LambdaExpression::Variable("f".to_string())),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                            Box::new(LambdaExpression::Abstraction(
                                "u".to_string(),
                                Box::new(LambdaExpression::Variable("x".to_string())),
                            )),
                        )),
                        Box::new(LambdaExpression::Abstraction(
                            "u".to_string(),
                            Box::new(LambdaExpression::Variable("u".to_string())),
                        )),
                    )),
                )),
            )),
        )
    );

    test_parse_lambda_expression!(
        function_application,
        "f(f x)",
        LambdaExpression::Application(
            Box::new(LambdaExpression::Variable("f".to_string())),
            Box::new(LambdaExpression::Application(
                Box::new(LambdaExpression::Variable("f".to_string())),
                Box::new(LambdaExpression::Variable("x".to_string())),
            )),
        )
    );


    #[test]
    fn test_balanced_expression() {
        indent_reset();
        let expr = String::from("(x y)");
        let parsed_expr = parse_string_balanced_expression(&expr);
        assert!(parsed_expr.is_ok());
        let (input, expr) = parsed_expr.unwrap();
        assert_eq!(input, "");
        assert_eq!(expr, "(x y)");
    }

    #[test]
    fn test_application() {
        indent_reset();
        let expr = String::from("f(x y)");
        let parsed_expr = parse_lambda_application(&expr);
        assert_eq!(
            parsed_expr,
            Ok((
                "",
                LambdaExpression::Application(
                    Box::new(LambdaExpression::Variable("f".to_string())),
                    Box::new(LambdaExpression::Application(
                        Box::new(LambdaExpression::Variable("x".to_string())),
                        Box::new(LambdaExpression::Variable("y".to_string())),
                    )),
                )
            ))
        );
    }
}
