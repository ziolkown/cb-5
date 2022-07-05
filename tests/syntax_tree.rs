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
    let text =
        fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-assignment.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          Assign(Integer)\n          [\n            VariableDeclaration(\"c\"),\n            VariableRef(\"c\", Integer),\n            Integer Literal: 0\n          ],\n          VariableDeclaration(\"b\"),\n          Assign(Integer)\n          [\n            VariableDeclaration(\"a\"),\n            VariableRef(\"a\", Integer),\n            Assign(Integer)\n            [\n              VariableRef(\"b\", Integer),\n              VariableRef(\"c\", Integer)\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_expr_01() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-01.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      Assign(Boolean)\n      [\n        VariableDeclaration(\"a\"),\n        VariableRef(\"a\", Boolean),\n        Boolean Literal: false\n      ],\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          Assign(Integer)\n          [\n            VariableDeclaration(\"c\"),\n            VariableRef(\"c\", Integer),\n            Integer Literal: 3\n          ],\n          Assign(Boolean)\n          [\n            VariableRef(\"a\", Boolean),\n            Leq\n            [\n              Integer Literal: 4,\n              VariableRef(\"c\", Integer)\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_expr_02() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-02.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          Assign(Integer)\n          [\n            VariableDeclaration(\"b\"),\n            VariableRef(\"b\", Integer),\n            Integer Literal: 1\n          ],\n          Assign(Integer)\n          [\n            VariableDeclaration(\"a\"),\n            VariableRef(\"a\", Integer),\n            UMinus(Integer)\n            [\n              VariableRef(\"b\", Integer)\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_expr_03() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-03.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          Assign(Boolean)\n          [\n            VariableDeclaration(\"b\"),\n            VariableRef(\"b\", Boolean),\n            Boolean Literal: true\n          ],\n          Assign(Boolean)\n          [\n            VariableDeclaration(\"c\"),\n            VariableRef(\"c\", Boolean),\n            Boolean Literal: false\n          ],\n          Assign(Boolean)\n          [\n            VariableDeclaration(\"a\"),\n            VariableRef(\"a\", Boolean),\n            LogOr\n            [\n              VariableRef(\"b\", Boolean),\n              VariableRef(\"c\", Boolean)\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_expr_04() {
    let text = fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-expr-04.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          VariableDeclaration(\"a\"),\n          Assign(Boolean)\n          [\n            VariableDeclaration(\"b\"),\n            VariableRef(\"b\", Boolean),\n            Boolean Literal: true\n          ],\n          Assign(Boolean)\n          [\n            VariableDeclaration(\"c\"),\n            VariableRef(\"c\", Boolean),\n            Boolean Literal: true\n          ],\n          Assign(Boolean)\n          [\n            VariableRef(\"a\", Boolean),\n            LogAnd\n            [\n              VariableRef(\"b\", Boolean),\n              VariableRef(\"c\", Boolean)\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_functioncall() {
    let text =
        fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-functioncall.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: test\n      [\n        Sequence\n        [\n          Assign(Integer)\n          [\n            VariableDeclaration(\"c\"),\n            VariableRef(\"c\", Integer),\n            Integer Literal: 2\n          ],\n          Return(Integer)\n          [\n            VariableRef(\"c\", Integer)\n          ]\n        ]\n      ],\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          FunctionCall: test\n          [\n            Sequence\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_syn_statement() {
    let text =
        fs::read_to_string("tests/testfiles/syntactical/test-cor-syn-statement.c-1").unwrap();
    let syntax_tree = parse(text.as_str()).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: blah\n      [\n        Sequence\n        [\n          Return(Float)\n          [\n            Float Literal: 3.14159\n          ]\n        ]\n      ],\n      Assign(Float)\n      [\n        VariableDeclaration(\"t\"),\n        VariableRef(\"t\", Float),\n        FunctionCall: blah\n        [\n          Sequence\n        ]\n      ],\n      FunctionDeclaration: main\n      [\n        Sequence\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_one_constant() {
    let text = "int x = 0;\nvoid main() {}";
    let syntax_tree = parse(text).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      Assign(Integer)\n      [\n        VariableDeclaration(\"x\"),\n        VariableRef(\"x\", Integer),\n        Integer Literal: 0\n      ],\n      FunctionDeclaration: main\n      [\n        Sequence\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_emtpy_main() {
    let text = "void main() {}";
    let syntax_tree = parse(text).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}

#[test]
fn cor_printf() {
    let text = r#"
    void main() {
        int a = 2;
        printf("test string");
        printf(a + 1);
    }"#;
    let syntax_tree = parse(text).unwrap();
    // println!("{}", syntax_tree);
    assert_eq!(String::from("Root\n[\n  Program\n  [\n    Sequence\n    [\n      FunctionDeclaration: main\n      [\n        Sequence\n        [\n          Assign(Integer)\n          [\n            VariableDeclaration(\"a\"),\n            VariableRef(\"a\", Integer),\n            Integer Literal: 2\n          ],\n          Print\n          [\n            String Literal: \"test string\"\n          ],\n          Print\n          [\n            Plus(Integer)\n            [\n              VariableRef(\"a\", Integer),\n              Integer Literal: 1\n            ]\n          ]\n        ]\n      ]\n    ]\n  ]\n]"), syntax_tree.print());
}
