%expect 0

%define api.parser.struct {Parser}
%define api.value.type {Value}
%define api.parser.check_debug { self.debug }
%define api.parser.generic {<'a /* 'fix quotes */>}

%define parse.error custom
%define parse.trace

%code use {
    // all use goes here
    use crate::{Lexer, AnalysisResult, Error};
    use super::bison_skeleton::{token::Token, value::Value::{self, Tree, Name}, loc::Loc};
    use super::structures::{symbol_table::{SymbolTable, SymbolClass, SymbolType}, syntax_tree::SyntaxTree};
    use super::syntax_c1::*;
}

%code parser_fields {
    errors: Vec<Error>,
    symbol_table: SymbolTable,
    syntax_tree: SyntaxTree<NodeValue>,
    /// Enables debug printing
    pub debug: bool,
}

%token
    AND           "&&"
    OR            "||"
    EQ            "=="
    NEQ           "!="
    LEQ           "<="
    GEQ           ">="
    LSS           "<"
    GRT           ">"
    KW_BOOLEAN    "bool"
    KW_DO         "do"
    KW_ELSE       "else"
    KW_FLOAT      "float"
    KW_FOR        "for"
    KW_IF         "if"
    KW_INT        "int"
    KW_PRINTF     "printf"
    KW_RETURN     "return"
    KW_VOID       "void"
    KW_WHILE      "while"
    CONST_INT     "integer literal"
    CONST_FLOAT   "float literal"
    CONST_BOOLEAN "boolean literal"
    CONST_STRING  "string literal"
    ID            "identifier"

// definition of association and precedence of operators
%left '+' '-' OR
%left '*' '/' AND
%nonassoc UMINUS

// workaround for handling dangling else
// LOWER_THAN_ELSE stands for a not existing else
%nonassoc LOWER_THAN_ELSE
%nonassoc KW_ELSE

%%

start:
	program {
		match self.symbol_table.get("main") {
			// The `start` rule has a different return type than the other rules, this is why this weird `unwrap_err()` construct is needed here.
                            None => {return Err(self.report_semantic_error("void main() doesn't exist").unwrap_err())}
                            Some(symbol) => {
                                if let SymbolClass::Function {parameters} = &symbol.symbol_class {
                            		// TODO: Handle remaining main related semantic errors
                            	        // The `start` rule has a different return type than the other rules.
                            	        // Therefore the returned error has to be rewrapped with
                            	        // return Err(self.report_semantic_error("ERROR").unwrap_err())
                                } else {
                                    // TODO: Handle remaining main related semantic errors
                                }

                                let mut program_node = program_node();
                                // Push the already parsed tree that is now on top of the stack
                                if let Tree(tree) = $program {
                                    program_node.push_node(tree);
                                } else {
                                    ast_parse_error();
                                }
                                // Add the parsed AST to the virtual root
                                self.syntax_tree.push_node(program_node);
                        }};

                $$ = Value::None;
	}
	;

/* see EBNF grammar for further information */
program:
	/* empty */
		{
		$$ = Tree(sequence_node());
		}
	| program[prog] declassignment[decl] ';'
		{
		let mut parent = $prog.unwrap_tree();
		let child = $decl.unwrap_tree();
		parent.push_node(child);
		$$ = Tree(parent);
		}
	| program[prog] functiondefinition[func]
	{
		let mut parent = $prog.unwrap_tree();
		let child = $func.unwrap_tree();
		parent.push_node(child);
		$$ = Tree(parent);
	}
functiondefinition:
	type ID[name] {
		let name = $name.unwrap_name();

                // TODO: Use symbol table

                $$ = Value::Tree(function_node(name));
	}
	'(' opt_parameterlist[params] ')' '{' statementlist[body] '}' {
	    let mut function_node = $3.unwrap_tree();
	    if let Tree(params) = $params {
	        function_node.push_node(params);
	    }
	    let body = $body.unwrap_tree();
	    function_node.push_node(body);

            // TODO: Use symbol table

	    $$ = Tree(function_node);
	}


opt_parameterlist:
	/* empty */
	{
	$$ = Value::None;
	}
	| parameterlist
	{
	$$ = $parameterlist;
	}

parameterlist:
	parameter
	{
	let mut sequence_node = sequence_node();
	sequence_node.push_node($parameter.unwrap_tree());
	$$ = Tree(sequence_node);
	}
	| parameter ',' parameterlist[list]
	{
	  let mut list = $list.unwrap_tree();
	  list.prepend_node($parameter.unwrap_tree());
	  $$ = Tree(list);
	}

parameter:
	type ID[name] {
		let name =$name.unwrap_name();
		// TODO: Check parameter type
	        // TODO: Use symbol table
	        $$ = Tree(parameter_node(name));
	}

functioncall:
	ID[name] '(' opt_argumentlist[args] ')' {
		// The variable must already be declared
		let name =$name.unwrap_name();
		match self.symbol_table.get(&name) {
			None => {
				let error = format!("undeclared symbol {}", &name);
				return self.report_semantic_error(&error);
			}
			Some(symbol) => {
				// It has been declared, but is it really a function?
				match &symbol.symbol_class {
				SymbolClass::Function {parameters} => {
					let mut call_node = function_call_node(name.clone(), symbol.symbol_type);
					let arg_node = $args.unwrap_tree();
					let arg_types: Vec<SymbolType> = arg_node.children().iter().map(|c| c.value().symbol_type()).collect();

					// TODO: Compare parameters with arguments



					call_node.push_node(arg_node);
					$$ = Tree(call_node);
				}
				_ => {
					// TODO: Handle symbol being not a function
				}
			}
		}
	}
}

opt_argumentlist:
	/* empty */
		{
		$$ = Value::Tree(sequence_node());
		}
	| argumentlist
	{
	$$ = $argumentlist;
	}

argumentlist:
	assignment[expr]
		{
		let mut sequence_node = sequence_node();
		sequence_node.push_node($expr.unwrap_tree());
		$$ = Tree(sequence_node);
		}
	| argumentlist[list] ',' assignment[elem]
		{
		let mut list_node = $list.unwrap_tree();
		list_node.push_node($elem.unwrap_tree());
		$$ = Tree(list_node);
		}

statementlist:
	/* empty */
		{
		$$ = Value::Tree(sequence_node());
		}
	| statementlist[list] statement[elem]
		{
		let mut list_node = $list.unwrap_tree();
		list_node.push_node($elem.unwrap_tree());
		$$ = Tree(list_node);
		}

block:
	'{' {
	// TODO: Use symbol_table
	$$ = Value::None;
	}
		statementlist[body]
	'}' {
	// TODO: Use symbol_table
 	$$ = $body;
	}

body:
	{
		// TODO: Use symbol_table
		$$ = Value::None;
	} statement {
		// TODO: Use symbol_table
		$$ = $statement;
	}

statement:
	  ifstatement
	  {
	  $$ = $ifstatement;
	  }
	| forstatement
	  {
	  $$ = $forstatement;
	  }
	| whilestatement
	  {
	  $$ = $whilestatement;
	  }
	| returnstatement ';'
	  {
	  $$ = $returnstatement;
	  }
	| dowhilestatement ';'
	  {
	  $$ = $dowhilestatement;
	  }
	| printf ';'
	  {
	  $$ = $printf;
	  }
	| declassignment ';'
	  {
	  $$ = $declassignment;
	  }
	| statassignment ';'
	  {
	  $$ = $statassignment;
	  }
	| functioncall ';'
	  {
	  $$ = $functioncall;
	  }
	| block
	  {
	  $$ = $block;
	  }

ifstatement:
	KW_IF '(' assignment[cond] ')' body[then] opt_else[else] {
	let condition_node = $cond.unwrap_tree();
	// TODO: Verify condition's type

	let mut if_node = if_node();
	if_node.push_node(condition_node);
	if_node.push_node($then.unwrap_tree());

	if let Tree(tree) = $else {
	    if_node.push_node(tree);
	}
	$$ = Tree(if_node);
	}

/* KW_ELSE has higher precedence, so an occuring 'else' will cause the */
/* execution of the second rule */
opt_else:
	/* empty */ %prec LOWER_THAN_ELSE
		{
		$$ = Value::None;
		}
	| KW_ELSE body[else]
		{
		$$ = $else;
		}

forstatement:
	KW_FOR '(' {
	self.symbol_table.enter_scope();
	$$ = Value::None;
	} declassignment[init] ';' expr[cond] ';' statassignment[step] ')' body {
	let mut for_node = for_node();
	let init_node = $init.unwrap_tree();
	let cond_node = $cond.unwrap_tree();
	let step_node = $step.unwrap_tree();
	let body_node = $body.unwrap_tree();


	// TODO: Verify condition type

	for_node.push_node(init_node);
	for_node.push_node(cond_node);
	for_node.push_node(step_node);
	for_node.push_node(body_node);

	$$ = Tree(for_node);
	self.symbol_table.leave_scope();
	}
	| KW_FOR '('
	{
		// This scope only belongs to the for statements head + body, no symbols will be defined, as
		// an already declared variable is used in this for variant
		self.symbol_table.enter_scope();
		$$ = Value::None;
	}
	statassignment[init] ';' expr[cond] ';' statassignment[step] ')' body {
	let mut for_node = for_node();
	let init_node = $init.unwrap_tree();
	let cond_node = $cond.unwrap_tree();
	let step_node = $step.unwrap_tree();
	let body_node = $body.unwrap_tree();

	// TODO: Verify condition type

	for_node.push_node(init_node);
	for_node.push_node(cond_node);
	for_node.push_node(step_node);
	for_node.push_node(body_node);

	$$ = Tree(for_node);
	self.symbol_table.leave_scope();
	}

dowhilestatement:
	KW_DO body KW_WHILE '(' assignment[cond] ')' {
	let condition_node = $cond.unwrap_tree();
	// TODO: Verify condition type
	$$ = combine(do_while_node(), Tree(condition_node), $body);
	}

whilestatement:
	KW_WHILE '(' assignment[cond] ')' body {
	let condition_node = $cond.unwrap_tree();
	// TODO: Verify condition type
	$$ = combine(while_node(), Tree(condition_node), $body);
	}

returnstatement:
	KW_RETURN {
	// It's a void return
	if SymbolType::Void == self.symbol_table.function_type().unwrap() {
	    $$ = Tree(return_node(SymbolType::Void));
	}
	// TODO: Handle invalid return type
	}
	| KW_RETURN assignment[expr] {
	// It's a value return
	let expr_node = $expr.unwrap_tree();
	let return_type = expr_node.value().symbol_type();
	let function_type = self.symbol_table.function_type().unwrap();

	if !match_types(function_type, return_type) {
	    // TODO: Handle non-matching return types
	} else if function_type != return_type {
		// The assignment performs a type cast, e.g. int to float
		// The cast node becomes the parent of the expression node
		let mut cast_node = cast_node(function_type);
		cast_node.push_node(expr_node);

		// The return node becomes the parent of the cast node
		let mut parent = return_node(return_type);
		parent.push_node(cast_node);

		$$ = Tree(parent);
	} else {
		// The return node becomes the parent of the expression node
		let mut parent = return_node(return_type);
		parent.push_node(expr_node);

		$$ = Tree(parent);
	}
	}

printf:
	KW_PRINTF '(' assignment[arg] ')'
		{
		let child = $arg.unwrap_tree();
		// TODO: Handle printf argument type
		let mut parent = print_node();
		parent.push_node(child);
		$$ = Tree(parent);
		}
	| KW_PRINTF '(' CONST_STRING[arg] ')'
		{
		let child = string_node($arg.unwrap_token().text.to_string());
		let mut parent = print_node();
		parent.push_node(child);
		$$ = Tree(parent);
		}

declassignment:
	type ID[name] {
		// It's a declaration without assignment
	        let name =$name.unwrap_name();
	        let symbol_type = $type.unwrap_type();
		let symbol = self.symbol_table.variable_symbol(name.clone(), symbol_type);
		match self.symbol_table.insert(symbol) {
		    Ok(_) => {
		        $$ = Tree(variable_node(name))
		    }
		    Err(error) => {
		    	self.add_error(error);
    			return Ok(Self::YYERROR);
    			}
		}
	}
	| type ID[name] {
		// It's a declaration with an immediate assignment. We have to declare the variable first
	      let name =$name.unwrap_name();
		let symbol_type = $type.unwrap_type();
		let symbol = self.symbol_table.variable_symbol(name.clone(), symbol_type);
		match self.symbol_table.insert(symbol) {
		    Ok(_) => {
			$$ = Tree(variable_node(name))
		    }
		    Err(error) => {
			self.add_error(error);
			    return Ok(Self::YYERROR);
			}
		}
	} '=' assignment[expr] {
		// We need the variable node from the just handled declaration
		let variable_node = $3.unwrap_tree();
		match self.handle_assignment(Name(variable_node.value().symbol_name().unwrap()), $expr) {
		    Ok(assignment_node) => {
		    	// Add the variable declaration to the assignment
		    	let mut assignment_node = assignment_node.unwrap_tree();
		        assignment_node.prepend_node(variable_node);
		        $$ = Tree(assignment_node);
		    },
		    Err(error) => {
		    return self.report_semantic_error(&error);},
		}
	}

type:
	KW_BOOLEAN {
	// Handling of keyword that defines the type of a function or variable
	$$ = Value::SymbolType(SymbolType::Boolean);
	}
	| KW_FLOAT {
	// Handling of keyword that defines the type of a function or variable
	$$ = Value::SymbolType(SymbolType::Float);
	}
	| KW_INT   {
	// Handling of keyword that defines the type of a function or variable
	$$ = Value::SymbolType(SymbolType::Integer);
	}
	| KW_VOID  {
	// Handling of keyword that defines the type of a function or variable
	$$ = Value::SymbolType(SymbolType::Void);
	}

statassignment:
	ID[name] '=' assignment[expr] {
		match self.handle_assignment($name, $expr) {
		    Ok(result) => $$ = result,
		    Err(error) => {
		    return self.report_semantic_error(&error);
		    }
		}
	}

assignment:
	ID[name] '=' assignment[expr] {
		// It's a variable assignment
		match self.handle_assignment($name, $expr) {
		    Ok(result) => $$ = result,
		    Err(error) => {
		    		    return self.report_semantic_error(&error);
		    		  }
		}
	}
	| expr
	{
	$$ = $expr;
	}

expr:
	simpexpr
		{
		$$ = $simpexpr;
		}
	| simpexpr[lhs] EQ  simpexpr[rhs]
		{
		match self.logical_operator(eq_node, $lhs, $rhs) {
		    Ok(value) => $$ = value,
		    Err(error) => return self.report_semantic_error(&error),
		}
		}
	| simpexpr[lhs] NEQ simpexpr[rhs]
		{
		match self.logical_operator(neq_node, $lhs, $rhs) {
                		    Ok(value) => $$ = value,
                		    Err(error) => return self.report_semantic_error(&error),
                		}
		}
	| simpexpr[lhs] LEQ simpexpr[rhs]
		{
		match self.logical_operator(leq_node, $lhs, $rhs) {
		    Ok(value) => $$ = value,
		    Err(error) => return self.report_semantic_error(&error),
		}
		}
	| simpexpr[lhs] GEQ simpexpr[rhs]
		{
		match self.logical_operator(geq_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] LSS simpexpr[rhs]
		{
		match self.logical_operator(lst_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] GRT simpexpr[rhs]
		{
		match self.logical_operator(grt_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}

simpexpr:
	simpexpr[lhs] '+' simpexpr[rhs]
		{
		match self.operator(plus_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] '-' simpexpr[rhs]
		{
		match self.operator(minus_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] OR simpexpr[rhs]
		{
		match self.logical_operator(log_or_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] '*' simpexpr[rhs]
		{
		match self.operator(times_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] '/' simpexpr[rhs]
		{
		match self.operator(divide_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| simpexpr[lhs] AND simpexpr[rhs]
		{
		match self.logical_operator(log_and_node, $lhs, $rhs) {
                    Ok(value) => $$ = value,
                    Err(error) => return self.report_semantic_error(&error),
                }
		}
	| '-' simpexpr[operand] %prec UMINUS {
	// It's an unary minus followed by an expression (the operand)

	// First, retrieve the operand's node
	let tree = $operand.unwrap_tree();
	match tree.value().symbol_type() {

	SymbolType::Integer => {
        	let mut u_minus = u_minus_node(SymbolType::Integer);
        	u_minus.push_node(tree);
        	$$ = Tree(u_minus);
	}
	SymbolType::Float  => {
	    let mut u_minus = u_minus_node(SymbolType::Float);
	    u_minus.push_node(tree);
	    $$ = Tree(u_minus);
	}
	_ => {
	    // TODO: Handle other types
	}
	}

	}
	| CONST_INT[val]
		{
		// It's an int literal; parse it
		    let token = $val.unwrap_token();
		    $$ = Tree(integer_node(token.text.parse().unwrap()));
		}
	| CONST_FLOAT[val]
		{
		// It's a float literal; parse it
		    let token = $val.unwrap_token();
		    $$ = Tree(float_node(token.text.parse().unwrap()));
		}
	| CONST_BOOLEAN[val]
		{
		// It's a boolean literal; parse it
		    let token = $val.unwrap_token();
		    $$ = Tree(boolean_node(token.text.parse().unwrap()));
		}
	| functioncall
		{
		// Return value of functioncall rule
		$$ = $functioncall;
		}
	| ID[name] {
		// The name of a variable is expected, as this is part of an expression
		// The variable must have been declared before.
		let name = $name.unwrap_token().text;

		match self.symbol_table.get(&name) {
		    None => {
			let error = format!("Undeclared symbol {}", name);
			return self.report_semantic_error(&error);
			}
		    Some(symbol) => {
		    	// TODO: Check for symbol class
			$$ = Tree(variable_ref_node(name, symbol.symbol_type));
		    }
		}
	}
	| '(' assignment ')'
	{
	// The return value is the value returned by the inner assignment
	$$ = $assignment;
	}

%%

impl<'a> Parser<'a> {
    /// "Sucess" status-code of the parser
    pub const ACCEPTED: i32 = -1;

    /// "Failure" status-code of the parser
    pub const ABORTED: i32 = -2;

    /// Constructor
    pub fn new(lexer: Lexer<'a /* 'fix quotes */>) -> Self {
        // This statement was added to manually remove a dead code warning for 'owned_value_at' which is auto-generated code
        Self::remove_dead_code_warning();
        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            debug: false,
            yyerrstatus_: 0,
            yylexer: lexer,
            errors: Vec::new(),
            symbol_table: SymbolTable::new(),
            syntax_tree: SyntaxTree::new(NodeValue::Root),
        }
    }

    /// Wrapper around generated `parse` method that also
    /// extracts the `errors` field and returns it.
    pub fn do_parse(mut self) -> AnalysisResult {
        self.parse();
        if self.errors.is_empty() {
	    Ok(self.syntax_tree)
        } else {
            Err(self.errors)
	}
    }

    /// Retrieve the next token from the lexer
    fn next_token(&mut self) -> Token {
        self.yylexer.yylex()
    }

    /// Print a syntax error and add it to the errors field
    fn report_syntax_error(&mut self, stack: &YYStack, yytoken: &SymbolKind, loc: YYLoc) {
        let token_name = yytoken.name();
        let error = format!("Unexpected token {} at {:?}", token_name, loc);
        eprintln!("Stack: {}\nError: {}", stack, error);
        self.errors.push(Error::Syntactical(error));
    }

    /// Print a semantic error and add it to the errors field
    fn report_semantic_error(&mut self, message: &str) -> Result<i32, ()>{
    	let error = format!("{}", message);
    	eprintln!("{}", error);
    	self.errors.push(Error::Semantic(error));
    	Err(())
    }

    /// Print an error and add it to the errors field
    fn add_error(&mut self, error: Error) {
        eprintln!("{}", error);
        self.errors.push(error);
    }

    /// Helper function for handling the creation of assign nodes that happens at multiple points in the parser.
    ///
    /// This function takes two Value instances as arguments. The first argument is assumed to contain the name 
    /// of the variable to which a value is assigned. The second argument is assumed to contain a SyntaxTree instance
    /// with an expression as root node (see C1 syntax).  
    fn handle_assignment(&mut self, name: Value, expr: Value) -> Result<Value, String> {
        // This is a static assignment to a global variable outside of a function.
	// The variable must already be declared
	let name = name.unwrap_name();
	match self.symbol_table.get(&name) {
	      None => {
		  return Err(format!("undeclared symbol {}", &name));
	      }
	      Some(symbol) => {
		  // It has been declared, but is it really a variable?
		  if symbol.is_function() {
		      return Err(format!("cannot assign to function {}", &name));
		  } else {
		      let node = expr.unwrap_tree();
		      let node_symbol_type = node.value().symbol_type();
		      if !match_types(symbol.symbol_type, node_symbol_type) {
			  return Err(format!("cannot assign {} to {}: {} = {}", node_symbol_type, symbol.symbol_type, &symbol.name, &node.value()));
		      } else if symbol.symbol_type != node_symbol_type {
			      // The assignment performs a type cast, e.g. int to float
			      let mut cast_node = cast_node(symbol.symbol_type);
			      cast_node.push_node(node);
			      return Ok(combine(assign_node(symbol.symbol_type), Tree(variable_ref_node(name, symbol.symbol_type)), Tree(cast_node)));
		      } else {
			      return Ok(combine(assign_node(symbol.symbol_type), Tree(variable_ref_node(name, symbol.symbol_type)), Tree(node)));
		      }
		  }
	      }
	  }
    }

    /// Create a new logical operator node with a left-hand-side (lhs) and right-hand-side (rhs).
    /// For convenience, lhs and rhs are assumed to be instances of Value that contain a SyntaxTree each
    fn logical_operator<T>(&mut self, operator_constructor: T, lhs: Value, rhs: Value) -> Result<YYValue, String>
    where T: Fn() -> SyntaxTree<NodeValue>
    {
	let lhs = lhs.unwrap_tree();
	let rhs = rhs.unwrap_tree();

	let lhs_type = lhs.value().symbol_type();
        let rhs_type = rhs.value().symbol_type();
        // Temporary operator node
        let parent = operator_constructor();
	let mut parent = match operator_type(parent.value(), lhs_type, rhs_type) {
	    Ok(_) => {
		operator_constructor()
	    }
	    Err(error) => {
	        return Err(error);
	    }
	};

	parent.push_node(lhs);
	parent.push_node(rhs);
	Ok(Tree(parent))
    }

    /// Create a new non-logical operator node with a left-hand-side (lhs) and right-hand-side (rhs).
    /// For convenience, lhs and rhs are assumed to be instances of Value that contain a SyntaxTree each
    fn operator<T>(&mut self, operator_constructor: T, lhs: YYValue, rhs: YYValue) -> Result<YYValue, String>
        where T: Fn(SymbolType) -> SyntaxTree<NodeValue>
        {
    	let lhs = lhs.unwrap_tree();
    	let rhs = rhs.unwrap_tree();

    	let lhs_type = lhs.value().symbol_type();
        let rhs_type = rhs.value().symbol_type();
        // Temporary operator node
        let parent = operator_constructor(SymbolType::Void);
    	let mut parent = match operator_type(parent.value(), lhs_type, rhs_type) {
    	    Ok(op_type) => {
    		operator_constructor(op_type)
    	    }
    	    Err(error) => {
    	        return Err(error);
    	    }
    	};

    	parent.push_node(lhs);
    	parent.push_node(rhs);
    	Ok(Tree(parent))
    }

    /// Helper function that removes a dead code warning, which would otherwise interfere with the correction of a submitted
    /// solution
    fn remove_dead_code_warning() {
    	let mut stack = YYStack::new();
    	let yystate: i32 = 0;
    	let yylval: YYValue = YYValue::new_uninitialized();
    	let yylloc: YYLoc = YYLoc { begin: 0, end: 0 };
        stack.push(yystate, yylval.clone(), yylloc);
    	let _ = stack.owned_value_at(0);
    }

}

/// Helper function for finding problems during AST parsing
fn ast_parse_error() {
    panic!("Was not able to parse the AST due to an incorrect parser implementation.");
}

/// Determine whether the lhs type can be assigned the rhs type. Return true if lhs is compatible with rhs (i.e., lhs = rhs;)
fn match_types(lhs: SymbolType, rhs: SymbolType) -> bool {
	lhs == rhs || (lhs == SymbolType::Float && rhs == SymbolType::Integer)
}

/// Determine the type of an operator, e.g., for `1 + 2` the `+`-operator has the result type _integer_.
fn operator_type(operator: &NodeValue, lhs: SymbolType, rhs: SymbolType) -> Result<SymbolType, String> {
    match operator {
        // Only numbers allowed, operator type is integer or float
        NodeValue::Plus(_) | NodeValue::Minus(_) | NodeValue::Times(_) | NodeValue::Divide(_) => {
            if lhs == SymbolType::Boolean || rhs == SymbolType::Boolean {
                Err(format!("Type boolean is not allowed for {} operator", operator))
            } else if  lhs == SymbolType::String || rhs == SymbolType::String{
                Err(format!("Type string is not allowed for {} operator", operator))
            } else if lhs == SymbolType::Float || rhs == SymbolType::Float {
                Ok(SymbolType::Float)
            } else {
                Ok(SymbolType::Integer)
            }
		}

        // Only numbers allowed, operator type is boolean
        NodeValue::Leq | NodeValue::Geq | NodeValue::Lst | NodeValue::Grt => {
            if (lhs == SymbolType::Integer || lhs == SymbolType::Float) && (rhs == SymbolType::Integer || rhs == SymbolType::Float) {
                Ok(SymbolType::Boolean)
            } else {
                Err(format!("type {} cannot be compared with {} using the {} operator", lhs, rhs, operator))
            }
        }

        // Any type allowed, operator type is boolean
        NodeValue::Eq | NodeValue::Neq => {
            if (lhs == SymbolType::Integer || lhs == SymbolType::Float) && (rhs == SymbolType::Integer || rhs == SymbolType::Float) {
                Ok(SymbolType::Boolean)
            } else if lhs == SymbolType::Boolean && rhs == SymbolType::Boolean {
                Ok(SymbolType::Boolean)
            } else {
                Err(format!("type {} cannot be compared with {} using the {} operator", lhs, rhs, operator))
            }
        }

        // Only boolean allowed, operator type is boolean
        NodeValue::LogOr | NodeValue::LogAnd => {
            if lhs == SymbolType::Boolean && rhs == SymbolType::Boolean {
                Ok(SymbolType::Boolean)
            } else {
                Err(format!("type {} cannot be compared with {} using the {} operator", lhs, rhs, operator))
            }
        }
        _ => {
            panic!("Invalid usage of combine function. Combine should only be used when the parent node is an operator")
        }
    }
}

/// Combine three SyntaxTree instances into a single SyntaxTree. This first SyntaxTree becomes the parent of the second
/// and third SyntaxTree instances, which become the first and second child respectively.
/// For convenient usage in bison actions, the SyntaxTree is wrapped in a Value::Tree variant.
fn combine(parent: SyntaxTree<NodeValue>, lhs: Value, rhs: Value) -> Value {
	let lhs = lhs.unwrap_tree();
	let rhs = rhs.unwrap_tree();

	let mut parent = parent;
	parent.push_node(lhs);
	parent.push_node(rhs);
	Tree(parent)
}