use cb_5::{AnalysisResult, Lexer, Parser};
use std::fs;

#[cfg(test)]
fn parse(input: &str) -> AnalysisResult {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.debug = false;
    parser.do_parse()
}

#[test]
fn cor_syn_assignment() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-assignment.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_expr_01() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-01.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_expr_02() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-02.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_expr_03() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-03.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_expr_04() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-04.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_functioncall() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-functioncall.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_syn_statement() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-statement.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn err_syn_assignment() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-assignment.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

#[test]
fn err_syn_expr_01() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-expr-01.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

#[test]
fn err_syn_expr_02() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-expr-02.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

#[test]
fn err_syn_functioncall() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-functioncall.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

#[test]
fn err_syn_if_semicolon() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-if+semicolon.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

#[test]
fn err_syn_return_semicolon() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-err-syn-return+semicolon.c-1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_err(), "Parser did not detect the error",);
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", result.err().unwrap());
}

