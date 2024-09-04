use core::fmt;
use std::{collections::HashMap, error, io::Empty, iter::Enumerate, path::Display};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Uso: cargo run -- ruta/a/tablas \"query\"");
    } else {
        let path: &String = &args[1]; 
        let text_query: &String = &args[2];
        match build_query(text_query, path) {
            Ok(x) => exec_query(&path, x),
            Err(x) => print_err(x)
        }
    }
}

fn exec_query(path: &String, query: query) {
    let mut file = std::fs::File::open(format!("{}/{}",path,query.operation));

}

fn build_query(text_query: &String, path: &String) -> Result<query, u32>{
    // Crea query valida o devuelve codigo de error.
    let args = text_to_vec(text_query);
    match full_check_query(&arg, &path) {
        Ok(x) =>  
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
fn full_check_query(args: &Vec<String>, path: &String) -> Result<u32, u32> {
    let mut error_code :u32 = 0;
    let mut codigo_query: queryType;
    match check_operation(&args) {
        Ok(x) => codigo_query = x,
        Err(x) => return Err(x)
    } 
    match check_table_exist(path, &args, codigo_query){
        Ok(x) => error_code = x,
        Err(x) => return Err(x)
    } 
    match 
}

fn check_operation(args: &Vec<String>) -> Result<queryType, u32> {
    let operation = &args[0];
    if operation.eq("DELETE") {
        check_format_delete(args)
    } else if operation.eq("INSERT" ) {
        check_format_insert(args)
    } else if operation.eq("SELECT") {
        check_format_select(args)
    } else if operation.eq("UPDATE") {
        Ok(queryType::UPDATE)
    } else {
        Err(1)          // Codigo: OPERACION INVALIDA
    }
}

fn check_format_delete(args: &Vec<String>) -> Result<queryType, u32> {
    if args[1].eq("FROM") && args[3].eq("WHERE"){
        Ok(queryType::DELETE)            
    } else {
        Err(3)      // Codigo: DELETE MAL FORMATEADO
    }
}

fn check_format_insert(args: &Vec<String>) -> Result<queryType, u32> {
    if args[1].eq("INTO") {
        if (args.len() - 3) % 2 != 0 || args.len() - 3 < 0 {
            Err(2)
        } else{
            let mut counter = 0;
            let correct_value_len = (args.len() - 3)/2;    // INSERT INTO a b c VALUES x y z. Len = 9. Correct len = 3.          
            while counter < correct_value_len && ! args[counter].eq("VALUES"){
                counter += 1;
            } 
    
            if counter == correct_value_len && args[counter].eq("VALUES") {
                Ok(queryType::INSERT)
            } else{
                Err(2)      // Codigo: INSERT MAL FORMATEADO
            }    
        }
    }
    else {
        Err(2)      // Codigo: INSERT MAL FORMATEADO
    }
}

fn check_format_select(args: &Vec<String>) -> Result<queryType, u32> {
    Ok(0)
}

fn check_table_exist(path: &String, args: &Vec<String>, query: queryType) -> Result<u32, u32>{
//    std::fs::File
}
