use crate::ast::*;

#[derive(Clone)]
pub enum ColumnType {
    TextType,
    IntType
}

pub trait Cell {
    fn as_text(&self) -> String;
    fn as_int(&self) -> i32;
}

pub struct Column {
    pub col_type: ColumnType,
    pub name: String,
}

pub struct Results {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Box<dyn Cell>>> 
}


pub const ERR_TABLE_DOES_NOT_EXIST: &str  = "Table does not exist";
pub const ERR_COLUMN_DOES_NOT_EXIST: &str = "Column does not exist";
//pub const ERR_INVALID_SELECT_ITEM: &str  = "Select item is not valid";
//pub const ERR_INVALID_DATATYPE: &str    = "Invalid datatype";
pub const ERR_MISSING_VALUES: &str      = "Missing values";

pub trait Backend {
    fn create_table(&mut self, create_table_statement: &CreateTableStatement) -> Result<bool, String>;
    fn insert(&mut self, insert_statement: &InsertStatement) -> Result<bool, String>;
    fn select(&self, select_statement: &SelectStatement) -> Result<Results, String>;
}