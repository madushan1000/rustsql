use parameterized::parameterized;
use rustsql::lexer::*;

#[parameterized(case = {
	(true, "105"),
	(true, "105 "),
	(true, "123."),
	(true, "123.145"),
	(true, "1e5"),
	(true, "1.e21"),
	(true, "1.1e2"),
	(true, "1.1e-2"),
	(true, "1.1e+2"),
	(true, "1e-1"),
	(true, ".1"),
	(true, "4."),
	(false, "e4"),
	(false, "1.."),
	(false, "1ee4"),
	(false, " 1")
})]
fn numeric(case: (bool, &str)) {
    let (is_number, value) = case;
    let (token, _, ok) = lex_numeric(value, Cursor::new());
    assert_eq!(is_number, ok);
    if ok {
        assert_eq!(token.unwrap().value, value.trim());
    }
}

#[parameterized(case = {
	(false, "a"),
	(true, "'abc'"),
	(true, "'a b'"),
	(true, "'a' "),
	(true, "'a '' b'"),
	(false, "'"),
	(false, ""),
	(false, " 'foo'")
})]
fn string(case: (bool, &str)) {
    let (is_string, mut value) = case;
    let (token, _, ok) = lex_string(value, Cursor::new());
    assert_eq!(ok, is_string);

    value = value.trim();

    if ok {
        assert_eq!(&value[1..value.len() - 1], token.unwrap().value);
    }
}

#[parameterized(case= {
	(true, "="),
	(true, "||")
})]
fn symbol(case: (bool, &str)) {
    let (is_symbol, mut value) = case;
    let (token, _, ok) = lex_symbol(value, Cursor::new());
    assert_eq!(is_symbol, ok);

    if ok {
        value = value.trim();
        assert_eq!(token.unwrap().value, value);
    }
}

#[parameterized(case= {
	(true,"a","a"),
	(true,"abc","abc"),
	(true,"abc ","abc"),
	(true,"\" abc \""," abc "),
	(true,"a9$","a9$"),
	(true,"userName","username"),
	(true,"\"userName\"","userName"),
	(false,"\"","\""),
	(false,"_sadsfa","_sadsfa"),
	(false,"9sadsfa","9sadsfa"),
	(false," abc"," abc")
})]
fn identifier(case: (bool, &str, &str)) {
    {
        let (is_identifier, input, value) = case;

        let (token, _, ok) = lex_identifier(input, Cursor::new());
        assert_eq!(is_identifier, ok);
        if ok {
            assert_eq!(value, token.unwrap().value);
        }
    }
}

#[parameterized(case= {
	(true,"select "),
	(true,"from"),
	(true,"as"),
	(true,"SELECT"),
	(true,"into"),
	(false," into"),
	(false,"flubbrety")
})]
fn keyword(case: (bool, &str)) {
    let (is_keyword, mut value) = case;

    let (token, _, ok) = lex_keyword(value, Cursor::new());
    assert_eq!(is_keyword, ok);
    if ok {
        value = value.trim();
        assert_eq!(value.to_lowercase(), token.unwrap().value);
    }
}

#[parameterized(case={
	("select a", 
		vec!{
			Token{loc: Location{col: 0, line: 0}, value: SELECT_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0}, value: "a".to_string(), 				kind: TokenKind::IdentifierKind}
	}, None),
	("select true",
		vec!{
			Token{loc: Location{col: 0, line: 0}, value: SELECT_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0}, value: "true".to_string(), 			kind: TokenKind::BooleanKind},
	}, None),
	("select 1",
		vec!{
			Token{loc: Location{col: 0, line: 0}, value: SELECT_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0}, value: "1".to_string(), 				kind:  TokenKind::NumericKind,}
	}, None),
	("select 'foo' || 'bar';", 
		vec!{
			Token{loc: Location{col: 0, line: 0}, 	value: SELECT_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0}, 	value: "foo".to_string(), 			kind: TokenKind::StringKind},
			Token{loc: Location{col: 13, line: 0}, 	value: CONCAT_SYMBOL.to_string(), 	kind: TokenKind::SymbolKind},
			Token{loc: Location{col: 16, line: 0}, 	value: "bar".to_string(), 			kind: TokenKind::StringKind},
			Token{loc: Location{col: 21, line: 0}, 	value: SEMICOLON_SYMBOL.to_string(),kind: TokenKind::SymbolKind},
	}, None),
	("CREATE TABLE u (id INT, name TEXT)", 
		vec!{
			Token{loc: Location{col: 0, line: 0}, 	value: CREATE_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0}, 	value: TABLE_KEYWORD.to_string(), 	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 13, line: 0}, 	value: "u".to_string(),  			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 15, line: 0}, 	value: "(".to_string(), 			kind: TokenKind::SymbolKind},
			Token{loc: Location{col: 16, line: 0}, 	value: "id".to_string(), 			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 19, line: 0}, 	value: "int".to_string(), 			kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 22, line: 0}, 	value: ",".to_string(), 			kind: TokenKind::SymbolKind},
			Token{loc: Location{col: 24, line: 0}, 	value: "name".to_string(), 			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 29, line: 0}, 	value: "text".to_string(), 			kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 33, line: 0}, 	value: ")".to_string(), 			kind: TokenKind::SymbolKind},
	}, None),
	("insert into users values (105, 233)",
		vec!{
			Token{loc: Location{col: 0, line: 0},	value: INSERT_KEYWORD.to_string(),	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0},	value: INTO_KEYWORD.to_string(),	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 12, line: 0},	value: "users".to_string(),			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 18, line: 0},	value: VALUES_KEYWORD.to_string(),	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 25, line: 0},	value: "(".to_string(),				kind: TokenKind::SymbolKind},
			Token{loc: Location{col: 26, line: 0},	value: "105".to_string(),			kind: TokenKind::NumericKind},
			Token{loc: Location{col: 30, line: 0},	value: ",".to_string(),				kind: TokenKind::SymbolKind},
			Token{loc: Location{col: 32, line: 0},	value: "233".to_string(),			kind: TokenKind::NumericKind},
			Token{loc: Location{col: 36, line: 0},	value: ")".to_string(),				kind: TokenKind::SymbolKind},
	}, None),
	("SELECT id FROM users;", 
		vec!{
			Token{loc: Location{col: 0, line: 0},	value: SELECT_KEYWORD.to_string(),	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 7, line: 0},	value: "id".to_string(),			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 10, line: 0},	value: FROM_KEYWORD.to_string(),	kind: TokenKind::KeywordKind},
			Token{loc: Location{col: 15, line: 0},	value: "users".to_string(),			kind: TokenKind::IdentifierKind},
			Token{loc: Location{col: 20, line: 0},	value: ";".to_string(),				kind: TokenKind::SymbolKind},
	}, None)
})]
fn lex(case: (&str, Vec<Token>, Option<String>)) {
    let (input, input_tokens, _input_err) = case;

    let tokens = lex(input).unwrap();
    assert_eq!(tokens.len(), input_tokens.len());
    assert_eq!(tokens, input_tokens);
}
