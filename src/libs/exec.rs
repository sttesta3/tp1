use std::fs::File;

use crate::query::query_type::QueryType;
use crate::query::Query;

pub fn exec_query(query: Query)  {
    let table = &query.table;
    match table {
        Some(f) => match std::fs::File::open(f) {
                Ok(file) => {
                    match &query.operation {
                        Some(op) => match op {
                            QueryType::DELETE => exec_query_delete(file, query),
                            QueryType::INSERT => exec_query_insert(file, query),
                            QueryType::SELECT => exec_query_select(file, query),
                            QueryType::UPDATE => exec_query_update(file, query),    
                        },
                        None => println!("Error en el programa")
                    }
                }, 
                Err(_) => println!("Error en el programa")
            } ,
        None => println!("Error en el programa")
    }
}

fn exec_query_delete(file: File, query: Query) {
    // TODO
}

fn exec_query_insert(file: File, query: Query) {
    // TODO
}

fn exec_query_select(file: File, query: Query) {
    // TODO
}

fn exec_query_update(file: File, query: Query) {
    // TODO
}
