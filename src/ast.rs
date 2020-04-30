
use crate::lexer::*;

#[derive(PartialEq, Debug)]
pub struct Ast{
	pub statements :Vec<Statement>,
}

#[derive(PartialEq, Debug)]
pub enum AstKind {
	SelectKind,
	CreateTableKind,
    InsertKind,
}

#[derive(PartialEq, Debug)]
pub struct Statement{
    pub select_statement: 		Option<SelectStatement>,
    pub create_table_statement: 	Option<CreateTableStatement>,
    pub insert_statement: 		Option<InsertStatement>,
    pub kind: 					AstKind
}

#[derive(PartialEq, Debug)]
pub struct InsertStatement{
    pub table:  Token,
    pub values: Vec<Expression>
}

#[derive(PartialEq, Debug)]
pub enum ExpressionKind{
	LiteralKind,
}

#[derive(PartialEq, Debug)]
pub struct Expression{
    pub literal: Token,
    pub kind: ExpressionKind
}

#[derive(PartialEq, Debug)]
pub struct ColumnDefinition{
    pub name: Token,
    pub datatype: Token
}

#[derive(PartialEq, Debug)]
pub struct CreateTableStatement{
    pub name: Token,
    pub cols: Vec<ColumnDefinition>
}

#[derive(PartialEq, Debug)]
pub struct SelectStatement{
    pub item: Vec<Expression>,
    pub from: Token
}