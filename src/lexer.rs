pub type Keyword = &'static str;

pub const SELECT_KEYWORD: Keyword = "select";
pub const FROM_KEYWORD: Keyword = "from";
pub const AS_KEYWORD: Keyword = "as";
pub const TABLE_KEYWORD: Keyword = "table";
pub const CREATE_KEYWORD: Keyword = "create";
pub const INSERT_KEYWORD: Keyword = "insert";
pub const INTO_KEYWORD: Keyword = "into";
pub const VALUES_KEYWORD: Keyword = "values";
pub const INT_KEYWORD: Keyword = "int";
pub const TEXT_KEYWORD: Keyword = "text";
pub const BOOL_KEYWORD: Keyword = "boolean";
pub const WHERE_KEYWORD: Keyword = "where";
pub const AND_KEYWORD: Keyword = "and";
pub const OR_KEYWORD: Keyword = "or";
pub const TRUE_KEYWORD: Keyword = "true";
pub const FALSE_KEYWORD: Keyword = "false";

pub type Symbol = &'static str;

pub const SEMICOLON_SYMBOL: Symbol = ";";
pub const ASTERISK_SYMBOL: Symbol = "*";
pub const COMMA_SYMBOL: Symbol = ",";
pub const LEFTPAREN_SYMBOL: Symbol = "(";
pub const RIGHTPAREN_SYMBOL: Symbol = ")";
pub const EQ_SYMBOL: Symbol = "=";
pub const NEQ_SYMBOL: Symbol = "<>";
pub const CONCAT_SYMBOL: Symbol = "||";
pub const PLUS_SYMBOL: Symbol = "+";

#[derive(Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    KeywordKind,
    SymbolKind,
    IdentifierKind,
    StringKind,
    NumericKind,
    BooleanKind,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub loc: Location,
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub pointer: usize,
    pub loc: Location,
}

type Lexer = fn(&str, Cursor) -> (Option<Token>, Cursor, bool);

impl PartialEq for Token {
    fn eq(&self, rhs: &Token) -> bool {
        self.value == rhs.value && self.kind == rhs.kind
    }
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            pointer: 0,
            loc: Location::new(),
        }
    }
}

impl Location {
    pub fn new() -> Location {
        Location { line: 0, col: 0 }
    }
}

impl Token {
    pub fn new() -> Token {
        Token{
            value: String::new(),
            kind: TokenKind::BooleanKind,
            loc: Location::new()
        }
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut cur = Cursor {
        pointer: 0,
        loc: Location { line: 0, col: 0 },
    };
    'lex: while &cur.pointer < &source.len() {
        let lexers: Vec<Lexer> = vec![
            lex_keyword,
            lex_symbol,
            lex_string,
            lex_numeric,
            lex_identifier,
        ];

        for l in lexers {
            if let (token, new_cursor, true) = l(&source, cur.clone()) {
                cur = new_cursor;
                match token {
                    Some(token) => tokens.push(token),
                    None => (),
                }

                continue 'lex;
            }
        }
        let mut hint = String::new();
        if tokens.len() > 0 {
            hint = " after ".to_string() + &tokens[tokens.len() - 1].value;
        }
        return Err(format!(
            "Unable to lex token{}, at {}:{}",
            hint, cur.loc.line, cur.loc.col
        ));
    }
    Ok(tokens)
}

pub fn lex_numeric(source: &str, ic: Cursor) -> (Option<Token>, Cursor, bool) {
    let mut cur = ic.clone();
    let mut period_found = false;
    let mut exp_marker_found = false;

    while cur.pointer < source.len() {
        let c = source.chars().nth(cur.pointer).unwrap();
        cur.loc.col += 1;

        let is_digit = c >= '0' && c <= '9';
        let is_period = c == '.';
        let is_exp_marker = c == 'e';

        if cur.pointer == ic.pointer {
            if !is_digit && !is_period {
                return (None, ic, false);
            }
            period_found = is_period;
            cur.pointer += 1;
            continue;
        }

        if is_period {
            if period_found {
                return (None, ic, false);
            }

            period_found = true;
            cur.pointer += 1;
            continue;
        }

        if is_exp_marker {
            if exp_marker_found {
                return (None, ic, false);
            }

            period_found = true;
            exp_marker_found = true;

            if cur.pointer == source.len() - 1 {
                return (None, ic, false);
            }

            let c_next = source.chars().nth(cur.pointer + 1).unwrap();
            if c_next == '-' || c_next == '+' {
                cur.pointer += 1;
                cur.loc.col += 1;
            }
            cur.pointer += 1;
            continue;
        }

        if !is_digit {
            break;
        }
        cur.pointer += 1;
    }

    if cur.pointer == ic.pointer {
        return (None, ic, false);
    }

    (
        Some(Token {
            value: source[ic.pointer..cur.pointer].to_string(),
            loc: cur.loc.clone(),
            kind: TokenKind::NumericKind,
        }),
        cur,
        true,
    )
}

pub fn lex_character_delimited(
    source: &str,
    ic: Cursor,
    delimiter: char,
) -> (Option<Token>, Cursor, bool) {
    let mut cur = ic.clone();

    if source[cur.pointer..].len() == 0 {
        return (None, ic, false);
    }
    if source.chars().nth(cur.pointer).unwrap() != delimiter {
        return (None, ic, false);
    }

    cur.loc.col += 1;
    cur.pointer += 1;

    let mut value = String::new();

    while cur.pointer < source.len() {
        let c = source.chars().nth(cur.pointer).unwrap();

        if c == delimiter {
            if cur.pointer + 1 >= source.len()
                || source.chars().nth(cur.pointer + 1).unwrap() != delimiter
            {
                cur.pointer += 1;
                cur.loc.col += 1;
                return (
                    Some(Token {
                        value: value.into(),
                        loc: ic.loc,
                        kind: TokenKind::StringKind,
                    }),
                    cur,
                    true,
                );
            } else {
                value.push(delimiter);
                cur.pointer += 1;
                cur.loc.col += 1;
            }
        }
        value.push(c);
        cur.loc.col += 1;
        cur.pointer += 1;
    }
    (None, ic, false)
}

pub fn lex_string(source: &str, ic: Cursor) -> (Option<Token>, Cursor, bool) {
    lex_character_delimited(source, ic, '\'')
}

pub fn longest_match(source: &str, ic: Cursor, options: &Vec<&str>) -> String {
    let mut value = String::new();
    let mut skip_list: Vec<usize> = vec![];
    let mut matched = String::new();

    let mut cur = ic.clone();

    while cur.pointer < source.len() {
        value.push(
            source
                .chars()
                .nth(cur.pointer)
                .unwrap()
                .to_ascii_lowercase(),
        );
        cur.pointer += 1;
        'matched: for (index, option) in options.iter().enumerate() {
            for skip in &skip_list {
                if index == *skip {
                    continue 'matched;
                }
            }

            if option.to_string() == value {
                skip_list.push(index);
                if option.len() > matched.len() {
                    matched = option.to_string();
                }

                continue;
            }

            let shares_prefix = value == option[..cur.pointer - ic.pointer];
            let too_long = value.len() > option.len();
            if too_long || !shares_prefix {
                skip_list.push(index);
            }
        }
        if skip_list.len() == options.len() {
            break;
        }
    }
    return matched;
}

pub fn lex_symbol(source: &str, ic: Cursor) -> (Option<Token>, Cursor, bool) {
    let c = source.chars().nth(ic.pointer).unwrap();
    let mut cur = ic.clone();
    cur.loc.col += 1;
    cur.pointer += 1;

    let symbols = vec![
        EQ_SYMBOL,
        NEQ_SYMBOL,
        CONCAT_SYMBOL,
        PLUS_SYMBOL,
        COMMA_SYMBOL,
        LEFTPAREN_SYMBOL,
        RIGHTPAREN_SYMBOL,
        SEMICOLON_SYMBOL,
        ASTERISK_SYMBOL,
    ];

    match c {
        '\n' => {
            cur.loc.line += 1;
            cur.loc.col = 0;
            (None, cur, true)
        }
        '\t' => (None, cur, true),
        ' ' => (None, cur, true),
        _ => match &longest_match(source, ic.clone(), &symbols)[..] {
            "" => (None, ic, false),
            matched => {
                cur.pointer = ic.pointer + matched.len();
                cur.loc.col = ic.loc.col + matched.len();
                (
                    Some(Token {
                        value: matched.to_string(),
                        loc: ic.loc,
                        kind: TokenKind::SymbolKind,
                    }),
                    cur,
                    true,
                )
            }
        },
    }
}

pub fn lex_identifier(source: &str, ic: Cursor) -> (Option<Token>, Cursor, bool) {
    if let (token, new_cursor, true) = lex_character_delimited(source, ic.clone(), '"') {
        return (token, new_cursor, true);
    }
    let mut cur = ic.clone();
    let mut c = source.chars().nth(cur.pointer).unwrap();

    let mut is_alphabetical = (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
    if !is_alphabetical {
        return (None, ic, false);
    }
    cur.pointer += 1;
    cur.loc.col += 1;

    let mut value = c.to_string();

    while cur.pointer < source.len() {
        c = source.chars().nth(cur.pointer).unwrap();

        is_alphabetical = (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
        let is_numeric = c >= '0' && c <= '9';
        if is_alphabetical || is_numeric || c == '$' || c == '_' {
            value.push(c);
            cur.loc.col += 1;
            cur.pointer += 1;
            continue;
        }
        break;
    }
    if value.len() == 0 {
        return (None, ic, false);
    }
    (
        Some(Token {
            value: value.to_lowercase(),
            loc: ic.loc,
            kind: TokenKind::IdentifierKind,
        }),
        cur,
        true,
    )
}

pub fn lex_keyword(source: &str, ic: Cursor) -> (Option<Token>, Cursor, bool) {
    let mut cur = ic.clone();
    let options = vec![
        SELECT_KEYWORD,
        FROM_KEYWORD,
        AS_KEYWORD,
        TABLE_KEYWORD,
        CREATE_KEYWORD,
        INSERT_KEYWORD,
        INTO_KEYWORD,
        VALUES_KEYWORD,
        INT_KEYWORD,
        TEXT_KEYWORD,
        BOOL_KEYWORD,
        WHERE_KEYWORD,
        AND_KEYWORD,
        OR_KEYWORD,
        TRUE_KEYWORD,
        FALSE_KEYWORD,
    ];

    let matched = longest_match(source, ic.clone(), &options);
    let kind: TokenKind;

    match &matched[..] {
        "" => return (None, ic, false),
        TRUE_KEYWORD | FALSE_KEYWORD => kind = TokenKind::BooleanKind,
        _ => kind = TokenKind::KeywordKind,
    }
    cur.pointer = ic.pointer + matched.len();
    cur.loc.col = ic.loc.col + matched.len();

    (
        Some(Token {
            value: matched.to_string(),
            kind,
            loc: ic.loc,
        }),
        cur,
        true,
    )
}
