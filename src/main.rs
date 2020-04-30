use std::io;
use std::io::Write;
mod memory;
mod backend;
mod lexer;
mod ast;
mod parser;

use ast::*;
use backend::*;

fn main() {
    let mut memory_backend = memory::MemoryBackend::new();
    println!("Welcome to gosql.");
    loop {
        print!("# ");
        io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_end();
        let ast = parser::parse(&line);
        if ast.is_err() {
            eprintln!("{:?}", ast);
            continue;
        }

        for stmt in ast.unwrap().statements {
            match stmt.kind {
                AstKind::CreateTableKind => {
                    memory_backend.create_table(&stmt.create_table_statement.unwrap()).unwrap();
                    println!("ok");
                },
                AstKind::InsertKind => {
                    memory_backend.insert(&stmt.insert_statement.unwrap()).unwrap();
                    println!("ok");
                },
                AstKind::SelectKind => {
                    let results = memory_backend.select(&stmt.select_statement.unwrap()).unwrap();
                    for col in &results.columns {
                        print!("| {} ", col.name);
                    }
                    println!("|");
                    println!("{}", "=".repeat(20));

                    for result in results.rows {
                        print!("|");
                        for (i, cell) in result.iter().enumerate() {
                            let col_type = &results.columns[i].col_type;
                            let s = match col_type {
                                ColumnType::IntType => cell.as_int().to_string(),
                                ColumnType::TextType => cell.as_text()
                            };
                            print!(" {} | ", s)
                        }
                        println!("");
                    }
                    println!("ok");
                }
            }
        }
    }
}