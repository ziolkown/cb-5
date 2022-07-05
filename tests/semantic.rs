use cb_5::{AnalysisResult, Error, Lexer, Parser};
use std::fs;

#[cfg(test)]
fn parse(input: &str) -> AnalysisResult {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.debug = false;
    parser.do_parse()
}

#[cfg(test)]
fn parse_expect_error(input: &str) {
    let result = parse(input);
    assert!(result.is_err(), "Parser did not detect the error",);
    let error_list = result.err().unwrap();
    let first_error = error_list.get(0).unwrap();
    match first_error {
        Error::Lexical(_) | Error::Syntactical(_) => {
            panic!("Expected semantic error found: {}", first_error)
        }
        _ => (),
    }
    println!("\n+++++++ Error Message Returned by Parser ++++++++");
    println!("{:?}\n", error_list);
}

#[test]
fn cor_sem_01() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-01.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_02() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-02.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_03() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-03.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_04() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-04.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_05() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-05.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_06() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-06.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_07() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-07.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_08() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-08.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn cor_sem_09() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-cor-sem-09.c1").unwrap();
    let result = parse(text.as_str());
    assert!(result.is_ok(), "Parse result: {:?}", result.err().unwrap());
}

#[test]
fn err_sem_assignmenttype_01() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_02() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_03() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_04() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_05() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_06() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_07() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-07.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_08() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-08.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_09() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-09.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_assignmenttype_10() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-assignmenttype-10.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_block_01() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-block-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_block_02() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-block-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_block_03() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-block-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_block_04() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-block-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_01() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_02() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_03() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_04() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_05() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_06() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_07() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-07.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_08() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-08.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_09() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-09.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_declaration_10() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-declaration-10.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_01() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_02() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_03() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_04() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_05() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_06() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_07() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-07.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_08() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-08.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_09() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-09.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_10() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-10.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_11() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-11.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_12() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-12.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_13() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-13.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_functioncall_14() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-functioncall-14.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_01() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_02() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_03() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_04() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_05() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_main_06() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-main-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_01() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_02() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_03() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_04() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_05() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_06() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_07() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-07.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_08() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-08.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_09() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-09.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_10() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-10.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_operationtype_11() {
    let text =
        fs::read_to_string("tests/testfiles/semantic/test-err-sem-operationtype-11.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_printf_01() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-printf-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_printf_02() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-printf-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_01() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-01.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_02() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-02.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_03() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-03.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_04() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-04.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_05() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-05.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_06() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-06.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_07() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-07.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_08() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-08.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_09() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-09.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_10() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-10.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_11() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-11.c1").unwrap();
    parse_expect_error(text.as_str());
}

#[test]
fn err_sem_statement_12() {
    let text = fs::read_to_string("tests/testfiles/semantic/test-err-sem-statement-12.c1").unwrap();
    parse_expect_error(text.as_str());
}
