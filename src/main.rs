use std::{collections::HashMap, io::Empty, iter::Enumerate};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Uso: cargo run -- ruta/a/tablas \"query\"");
    } else {
        let path: &String = &args[1]; 
        let text_query: &String = &args[2];
        let query_result = build_query(text_query, path);
    }
}

fn build_query(text_query: &String, path: &String) -> Result<query, u32>{
    let args = text_to_vec(text_query);
    match full_check_query(&args) {
        Ok(x) => Ok(0),
        Err(x) => Err(x)
    }

}

fn text_to_vec(text_query: &String) -> Vec<String> {
    let mut resultado: Vec<String> = Vec::new();
    let mut split = text_query.split(' ');

    let mut element_opt = split.next();
    while element_opt.is_some() {
        match element_opt {
            Some(x) => resultado.push(x.to_string()),    // TODO: Check si esto esta bien 
            None => continue
        }
       element_opt = split.next();
    }

    resultado
}

fn full_check_query(args: &Vec<String>) -> Result<u32, u32> {
    let mut result :u32 = 0;
    match check_operation(&args[0]) {
        Ok(x) => result += x,
        Err(x) => result += x
    }
    match 
}

fn check_operation(operation: &String) -> Result<u32, u32> {
    if operation.eq("DELETE") || operation.eq("INSERT") || operation.eq("SELECT") || operation.eq("UPDATE") {
        Ok(0)
    } else {
        Err(1)
    }
}

enum queryType {
    DELETE,
    INSERT,
    SELECT,
    UPDATE
}

struct query {
    operation:          queryType,
    tabla:              String,                 // DELETE, INSERT, SELECT, UPDATE
    selectores:         Option<Vec<String>>,    // INSERT, SELECT, UPDATE
    where_condition:    Option<String>,         // DELETE (siempre), SELECT (a veces), UPDATE (siempre) 
    order_by:           Option<String>,         // SELECT (a veces)
    values:             Option<String>          // INSERT y UPDATE (en update es set)
}
