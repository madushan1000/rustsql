
use crate::lexer::*;
use crate::ast::*;

fn token_from_keyword(k: Keyword) -> Token {
    Token {
        kind: TokenKind::KeywordKind,
        value: k.to_string(),
        loc: Location::new()
    }
}

fn token_from_symbol(s: Symbol) -> Token {
    Token {
        kind: TokenKind::SymbolKind,
        value: s.to_string(),
        loc: Location::new()
    }
}

fn expect_token(tokens: &Vec<Token>, cursor: usize, t: Token) -> bool {
    if cursor >= tokens.len() {
        return false
    }
    t == tokens[cursor]
}

fn help_message(tokens: &Vec<Token>, cursor: usize, msg: String) {
    let c: &Token;
    if cursor < tokens.len() {
        c = &tokens[cursor];
    } else {
        c = &tokens[cursor - 1];
    }
    println!("[{},{}]: {}, got: {}", c.loc.line, c.loc.col, msg, c.value);
}

pub fn parse(source: &str) -> Result<Ast, String> {
    let tokens = lex(&source)?;
    let mut a = Ast{statements: vec!{}};

    let mut cursor = 0;

    while cursor < tokens.len() {
        if let(statement, new_cursor, true) = parse_statement(&tokens, cursor, token_from_symbol(SEMICOLON_SYMBOL)){
            cursor = new_cursor;
            a.statements.push(statement.unwrap());

            let mut at_least_one_semicolon = false;

            loop {
                if expect_token(&tokens, cursor, token_from_symbol(SEMICOLON_SYMBOL)){
                    cursor += 1;
                    at_least_one_semicolon = true;
                } else{
                    break
                }
            } 

            if !at_least_one_semicolon {
                help_message(&tokens, cursor, "Expected semi-colon delimiter between statements".to_string());
                return Err("Missing semi-colon between statements".to_string());
            }
        } else{
            help_message(&tokens, cursor, "Expected statement".to_string());
            return Err("Failed to parse, expected statement".to_string());
        }
    }
    Ok(a)
}

fn parse_statement(tokens: &Vec<Token>, initial_cursor: usize, _delimiter: Token) -> (Option<Statement>, usize, bool) {
    let cursor = initial_cursor;
    let _test = vec![1,2,3,4];

    let semicolon_token = token_from_symbol(SEMICOLON_SYMBOL);

    if let(select, new_cursor, true) = parse_select_statement(tokens, cursor, &semicolon_token){
        return (Some(Statement{
            kind: AstKind::SelectKind,
            select_statement: select,
            create_table_statement: None,
            insert_statement: None,
        }), new_cursor, true);
    }

    if let(insert, new_cursor, true) = parse_insert_statement(tokens, cursor, &semicolon_token){
        return (Some(Statement{
            kind: AstKind::InsertKind,
            select_statement: None,
            create_table_statement: None,
            insert_statement: insert
        }), new_cursor, true);
    }

    if let(create_table, new_cursor, true) = parse_create_table_statement(tokens, cursor, &semicolon_token){
        return (Some(Statement{
            kind: AstKind::CreateTableKind,
            select_statement: None,
            create_table_statement: create_table,
            insert_statement: None
        }), new_cursor, true);
    }

    (None, initial_cursor, false)
}

fn parse_select_statement(tokens: &Vec<Token>, initial_cursor: usize, delimiter: &Token) -> (Option<SelectStatement>, usize, bool){
    let mut cursor = initial_cursor;

    if !expect_token(&tokens, cursor, token_from_keyword(SELECT_KEYWORD)){
        return (None, initial_cursor, false);
    }
    cursor += 1;

    let mut select = SelectStatement{
        item: vec!{},
        from: Token::new()
    };

    if let(expressions, new_cursor, true) = parse_expressions(&tokens, cursor, vec!{&token_from_keyword(FROM_KEYWORD), delimiter}){
        select.item = expressions.unwrap();
        cursor = new_cursor;

        if expect_token(tokens, cursor, token_from_keyword(FROM_KEYWORD)){
            cursor += 1;

            if let(from, new_cursor, true) = parse_token(&tokens, cursor, TokenKind::IdentifierKind){
                select.from = from.unwrap().clone();
                cursor = new_cursor;
            } else{
                help_message(&tokens, cursor, "Expected FROM token".to_string());
                return (None, initial_cursor, false);
            }
        }
    } else{
        return (None, initial_cursor, false);
    }
    return (Some(select), cursor, true);
}

fn parse_token(tokens: &Vec<Token>, initial_cursor: usize, kind: TokenKind) -> (Option<&Token>, usize, bool){
    let cursor = initial_cursor;

    if cursor >= tokens.len() {
        return (None, initial_cursor, false);
    }

    let current = &tokens[cursor];
    if current.kind == kind {
        return (Some(current), cursor + 1, true);
    }
    return (None, initial_cursor, false);
}

fn parse_expressions(tokens: &Vec<Token>, initial_cursor: usize, delimiters: Vec<&Token>) -> (Option<Vec<Expression>>, usize, bool){
    let mut cursor = initial_cursor;

    let mut expressions:Vec<Expression> = vec!{};
    'outer: loop{
        if cursor >= tokens.len() {
            return (None, initial_cursor, false);
        }

        let current = &tokens[cursor];

        for delimiter in &delimiters {
            if *delimiter == current {
                break 'outer;
            }
        }

        if expressions.len() > 0 {
            if !expect_token(tokens, cursor, token_from_symbol(COMMA_SYMBOL)) {
                help_message(tokens, cursor, "Expected comma".to_string());
                return (None, initial_cursor, false);
            }
            cursor += 1;
        }

        if let(expression, new_cursor, true) = parse_expression(tokens, cursor, token_from_symbol(COMMA_SYMBOL)) {
            cursor = new_cursor;
            expressions.push(expression.unwrap());
        } else{
            help_message(tokens, cursor, "Expected expression".to_string());
            return (None, initial_cursor, false);
        }
    }
    (Some(expressions), cursor, true)
}

fn parse_expression(tokens: &Vec<Token>, initial_cursor: usize, _: Token) -> (Option<Expression>, usize, bool) {
    let cursor = initial_cursor;

    let kinds = vec!{TokenKind::IdentifierKind, TokenKind::NumericKind, TokenKind::StringKind};

    for kind in kinds {
        if let(t, new_cursor, true) = parse_token(tokens, cursor, kind) {
            return (Some(Expression{
                literal: t.unwrap().clone(),
                kind: ExpressionKind::LiteralKind
            }), new_cursor, true);
        }
    }
    return (None, initial_cursor, false);
}
fn parse_insert_statement(tokens: &Vec<Token>, initial_cursor: usize, _delimiter: &Token) -> (Option<InsertStatement>, usize, bool){
    let mut cursor = initial_cursor;

    if !expect_token(tokens, cursor, token_from_keyword(INSERT_KEYWORD)){
        return (None, initial_cursor, false);
    }
    cursor += 1;

    if !expect_token(tokens, cursor, token_from_keyword(INTO_KEYWORD)){
        help_message(tokens, cursor, "Expected into".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    let (table, new_cursor, ok) = parse_token(tokens, cursor, TokenKind::IdentifierKind);
    if !ok {
        help_message(tokens, cursor, "Expected table name".to_string());
        return (None, initial_cursor, false)
    }
    cursor = new_cursor;

    if !expect_token(tokens, cursor, token_from_keyword(VALUES_KEYWORD)){
        help_message(tokens, cursor, "Expected VALUES".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    if !expect_token(tokens, cursor, token_from_symbol(LEFTPAREN_SYMBOL)) {
        help_message(tokens, cursor, "Expected left paren".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    let (values, new_cursor, ok) = parse_expressions(tokens, cursor, vec!{&token_from_symbol(RIGHTPAREN_SYMBOL)});
    if !ok {
        return (None, initial_cursor, false);
    }
    cursor = new_cursor;

    if !expect_token(tokens, cursor, token_from_symbol(RIGHTPAREN_SYMBOL)) {
        help_message(tokens, cursor, "Expected right paren".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    (Some(InsertStatement{
        table: table.unwrap().clone(),
        values: values.unwrap(),
    }), cursor, true)
}

fn parse_create_table_statement(tokens: &Vec<Token>, initial_cursor: usize, _delimiter: &Token) -> (Option<CreateTableStatement>, usize, bool) {
    let mut cursor = initial_cursor;

    if !expect_token(tokens, cursor, token_from_keyword(CREATE_KEYWORD)) {
        return (None, initial_cursor, false);
    }
    cursor += 1;

    if !expect_token(tokens, cursor, token_from_keyword(TABLE_KEYWORD)) {
        return (None, initial_cursor, false);
    }
    cursor += 1;

    let (name, new_cursor, ok) = parse_token(tokens, cursor, TokenKind::IdentifierKind);
    if !ok {
        help_message(tokens, cursor, "Expected table name".to_string());
        return (None, initial_cursor, false);
    }
    cursor = new_cursor;

    if !expect_token(tokens, cursor, token_from_symbol(LEFTPAREN_SYMBOL)) {
        help_message(tokens, cursor, "Expected left parenthesis".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    let (cols, new_cursor, ok) = parse_column_definitions(tokens, cursor, token_from_symbol(RIGHTPAREN_SYMBOL));
    if !ok {
        return (None, initial_cursor, false);
    }
    cursor = new_cursor;

    if !expect_token(tokens, cursor, token_from_symbol(RIGHTPAREN_SYMBOL)) {
        help_message(tokens, cursor, "Expected right parenthesis".to_string());
        return (None, initial_cursor, false);
    }
    cursor += 1;

    return (Some(CreateTableStatement{
        name: name.unwrap().clone(),
        cols: cols.unwrap(),
    }), cursor, true)
}

fn parse_column_definitions(tokens: &Vec<Token>, initial_cursor: usize, delimiter: Token) -> (Option<Vec<ColumnDefinition>>, usize, bool) {
    let mut cursor = initial_cursor;

    let mut cds:Vec<ColumnDefinition> = vec!{};

    loop {
        if cursor >= tokens.len() {
            return (None, initial_cursor, false);
        }

        let current = &tokens[cursor];
        if delimiter == *current {
            break
        }

        if cds.len() > 0 {
            if !expect_token(tokens, cursor, token_from_symbol(COMMA_SYMBOL)) {
                help_message(tokens, cursor, "Expected comma".to_string());
                return (None, initial_cursor, false);
            }

            cursor += 1;
        }

        let (id, new_cursor, ok) = parse_token(tokens, cursor, TokenKind::IdentifierKind);
        if !ok {
            help_message(tokens, cursor, "Expected column name".to_string());
            return (None, initial_cursor, false);
        }
        cursor = new_cursor;

        let (ty, new_cursor, ok) = parse_token(tokens, cursor, TokenKind::KeywordKind);
        if !ok {
            help_message(tokens, cursor, "Expected column type".to_string());
            return (None , initial_cursor, false);
        }
        cursor = new_cursor;

        cds.push(ColumnDefinition{
            name: id.unwrap().clone(),
            datatype: ty.unwrap().clone(),
        });
    }
    (Some(cds), cursor, true)
}