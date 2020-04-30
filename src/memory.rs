use core::ops::Deref;
use crate::backend::*;
use crate::lexer::*;
use crate::ast::*;
use std::collections::BTreeMap;

use std::convert::TryInto;

#[derive(Clone, Debug)]
struct MemoryCell(Vec<u8>);

impl Deref for MemoryCell {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Cell for MemoryCell {
    fn as_int(&self) -> i32 {
        i32::from_be_bytes(self[..].try_into().unwrap())
    }

    fn as_text(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }
}

struct Table {
    columns: Vec<String>,
    column_types: Vec<ColumnType>,
    rows: Vec<Vec<MemoryCell>>
}

pub struct MemoryBackend {
    tables: BTreeMap<String, Table>
}

impl MemoryBackend{
    pub fn new() -> MemoryBackend {
        MemoryBackend{
            tables: BTreeMap::new()
        }
    }

    fn token_to_cell(token: &Token) -> MemoryCell {
        match token.kind {
            TokenKind::NumericKind => MemoryCell(token.value.parse::<i32>().unwrap().to_be_bytes().to_vec()),
            TokenKind::StringKind => MemoryCell(token.value.as_bytes().to_vec()),
            _ => panic!("unknown kind")
        }
    
    }
}

impl Backend for MemoryBackend {
    fn create_table(&mut self, crt: &CreateTableStatement) -> Result<bool, String> {
        let mut table = Table{
            columns: vec!{},
            column_types: vec!{},
            rows: vec!{},
        };

        for column in &crt.cols {
            table.columns.push(column.name.value.clone());

            let datatype = match &column.datatype.value[..] {
                "int" => ColumnType::IntType,
                "text" => ColumnType::TextType,
                _ => panic!("wrong type")
            };

            table.column_types.push(datatype);
        }

        self.tables.insert(crt.name.value.clone(), table);
        
        Ok(true)
    }

    fn insert(&mut self, inst: &InsertStatement) -> Result<bool, String> { 
        match self.tables.get_mut(&inst.table.value) {
            Some(table) => {
                if inst.values.len() != table.columns.len() {
                    return Err(ERR_MISSING_VALUES.to_string());
                }

                let mut row: Vec<MemoryCell> = vec!{};

                for value in &inst.values {
                    if value.kind != ExpressionKind::LiteralKind {
                        println!("Skipping non-literal.");
                        continue;
                    }
                    row.push(MemoryBackend::token_to_cell(&value.literal))
                }
                table.rows.push(row);
                Ok(true)
            },
            None => Err(ERR_TABLE_DOES_NOT_EXIST.to_string())
        }
    }

    fn select(&self, slct: &SelectStatement) -> Result<Results, String> {
        match self.tables.get(&slct.from.value) {
            None => Err(ERR_TABLE_DOES_NOT_EXIST.to_string()),
            Some(table) => {
                let mut results: Vec<Vec<Box<dyn Cell>>> = vec!{};
                let mut columns: Vec<Column> = vec!{};

                for row in &table.rows {
                    let mut result: Vec<Box<dyn Cell>> = vec!{};

                    for exp in &slct.item {
                        if exp.kind != ExpressionKind::LiteralKind {
                            println!("Skipping non-literal expression.");
                            continue;
                        }

                        let lit = &exp.literal;

                        if lit.kind == TokenKind::IdentifierKind {
                            let mut found = false;

                            for (j, col) in table.columns.iter().enumerate() {
                                if *col == lit.value {
                                    if columns.len() < table.columns.len() {
                                        columns.push(Column{
                                            col_type: table.column_types[j].clone(),
                                            name: lit.value.clone()
                                        });
                                    }

                                    result.push(Box::new(row[j].clone()) as Box<dyn Cell>);
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                return Err(ERR_COLUMN_DOES_NOT_EXIST.to_string());
                            }

                            continue;
                        }

                        return Err(ERR_COLUMN_DOES_NOT_EXIST.to_string());
                    }

                    results.push(result)
                }
                return Ok(Results{
                    columns,
                    rows: results
                });
            }
        }
    }
}