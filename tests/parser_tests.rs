use parameterized::parameterized;
use rustsql::parser::*;
use rustsql::ast::*;
use rustsql::lexer::*;

#[parameterized(case = {
	("CREATE TABLE users (id INT, name TEXT);",
		Ast{
			statements: vec!{
				Statement{
					kind: AstKind::CreateTableKind,
					insert_statement: None,
					select_statement: None,
					create_table_statement: Some(CreateTableStatement{
						name: Token{
							loc: Location{col: 13, line: 0},
							kind: TokenKind::IdentifierKind,
							value: "users".to_string()
						},
						cols: vec!{
							ColumnDefinition{
								name: Token{
									loc: Location{col: 20, line: 0},
									kind: TokenKind::IdentifierKind,
									value: "id".to_string()
								},
								datatype: Token{
									loc: Location{col: 23, line: 0},
									kind: TokenKind::KeywordKind,
									value: "int".to_string()
								}
							},
							ColumnDefinition{
								name: Token{
									loc:   Location{col: 28, line: 0},
									kind:  TokenKind::IdentifierKind,
									value: "name".to_string(),
								},
								datatype: Token{
									loc:   Location{col: 33, line: 0},
									kind:  TokenKind::KeywordKind,
									value: "text".to_string(),
								}
							}
						}
					})
				}
			}
		}),

})]
fn parse(case: (&str, Ast)){
	let (source, ast) = case;
	println!("(Parser) Testing: {}", source);
	let output = parse(source).unwrap();
	assert_eq!(ast, output);
}