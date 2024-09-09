use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::condition::condition_type::ConditionOperator;
use crate::condition::build_condition ;
//use crate::condition::{add_node_to_tree, build_complex_condition, build_condition, build_empty_complex_condition, get_where_columns, tree_check, ComplexCondition, Condition};
// use crate::condition::Condition;
use crate::libs::error;
use crate::query::{Query, DELETE_MIN_LEN, INSERT_MIN_LEN, SELECT_MIN_LEN, UPDATE_MIN_LEN};
use crate::query::query_type::QueryType;
use crate::query::build_empty_query;

use super::error::SELECT_MAL_FORMATEADO;

pub fn build_query(text_query: &String, path: &String) -> Result<Query, u32> {
    // Full "Compilation" process of query. Tokenization, sintactic analysis, semantic and build 
    let args = text_to_vec(&text_query);  
    match vec_to_query(&args, path) {               
        Ok(x) => return validate_query(x),  
        Err(x) => return Err(x)
    }
}

fn merge_table_and_path(path: &String, table: &String) -> String {
    let mut table_full: String = path.to_string();
    table_full.push('/');
    table_full.push_str(table); 
    table_full.push_str(".csv");

    table_full
}

fn separate_args_delete(args: &Vec<String>, path: &String, result: &mut Query) -> Result<u32,u32> {  
    if args.len() < DELETE_MIN_LEN {
        return Err(error::DELETE_MAL_FORMATEADO);
    } else {
        result.table = Some(merge_table_and_path(path, &args[2]));

        if args.len() != DELETE_MIN_LEN {
            return Err(error::DELETE_MAL_FORMATEADO)
        } else {
            /* TODO: Code for complex conditions
            match add_node_to_tree(args, &mut 4, build_empty_complex_condition()) {
                Ok(cond) => result.where_condition = Some(cond),
                Err(x) => return Err(x)
            }
            */

            if args[5].eq("<") {
                result.where_condition = Some(build_condition(args[4].to_string(), args[6].to_string(), ConditionOperator::Minor))            
            } else if args[5].eq("<=") {
                result.where_condition = Some(build_condition(args[4].to_string(), args[6].to_string(), ConditionOperator::MinorEqual))            
            } else if args[5].eq("=") {
                result.where_condition = Some(build_condition(args[4].to_string(), args[6].to_string(), ConditionOperator::Equal))            
            } else if args[5].eq(">=") {
                result.where_condition = Some(build_condition(args[4].to_string(), args[6].to_string(), ConditionOperator::HigherEqual))            
            } else if args[5].eq(">") {
                result.where_condition = Some(build_condition(args[4].to_string(), args[6].to_string(), ConditionOperator::Higher))            
            } else {
                return Err(error::WHERE_MAL_FORMATEADO)
            }
        }
        Ok(0)    
    }
}

fn separate_args_insert(args: &Vec<String>, path: &String, result: &mut Query) -> Result<u32,u32> {
    if args.len() < INSERT_MIN_LEN {
        return Err(error::INSERT_MAL_FORMATEADO)
    } else {
        result.table = Some(merge_table_and_path(path, &args[2]));   

        let mut columns: Vec<String> = Vec::new();
        let mut counter = 3; 
        while counter < args.len()/2 && ! args[counter].eq("VALUES") {
            columns.push(args[counter].to_string());
            counter += 1;
        }
    
        counter += 1;
        let mut values: Vec<String> = Vec::new();
        while counter < args.len() {
            values.push(args[counter].to_string());
            counter += 1;
        }
    
        result.columns = Some(columns);
        result.values = Some(values);
        Ok(0)    
    }
}

fn separate_args_update(args: &Vec<String>, path: &String, result: &mut Query) -> Result<u32,u32> {
    if args.len() < UPDATE_MIN_LEN {
        result.table = Some(merge_table_and_path(path, &args[1]));   

        let mut counter = 3; 
        let mut columns: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
    
        while counter < args.len() && !args[counter].eq("WHERE") {
            if counter % 3 == 0 {
                columns.push(args[counter].to_string());
            } else if ( counter % 3 ) == 2 {
                values.push(args[counter].to_string());
            } 
            counter += 1;
        }
    
        result.columns = Some(columns);
        result.values = Some(values);

        /* TODO: Code for complex conditions 
        counter += 1;   
        match add_node_to_tree(args, &mut counter, build_empty_complex_condition()) {
            Ok(cond) => result.where_condition = Some(cond),
            Err(x) => return Err(x)
        }
        */
        Ok(0)    
    } else {
        return Err(error::UPDATE_MAL_FORMATEADO)
    }
}

fn separate_args_select(args: &Vec<String>, path: &String, result: &mut Query) -> Result<u32,u32> { 
    if args.len() < SELECT_MIN_LEN {
        return Err(error::SELECT_MAL_FORMATEADO)
    } else {
        let mut columns: Vec<String> = Vec::new();
        let mut counter = 1; 
        while counter < args.len() && !args[counter].eq("FROM") {
            columns.push(args[counter].to_string());
            counter += 1;
        }
        counter += 1;

        
        if ! &columns[0].eq("*") {
            result.columns = Some(columns);
        }

        result.table = Some(merge_table_and_path(path, &args[counter]));
        counter += 1;
        
        if counter == args.len() {
            Ok(0)
        } else {
            /* TODO: Code for complex conditions
            match add_node_to_tree(args, &mut  counter, build_empty_complex_condition()) {
                Ok(cond) => result.where_condition = Some(cond),
                Err(x) => return Err(x)
            }
            */
            if counter + 3 > args.len(){
                return Err(SELECT_MAL_FORMATEADO);
            } else if args[counter].eq("WHERE") {
                counter += 1;
                if args[counter + 1].eq("<") {
                    result.where_condition = Some(build_condition(args[counter].to_string(), args[counter + 2].to_string(), ConditionOperator::Minor))            
                } else if args[counter + 1].eq("<=") {
                    result.where_condition = Some(build_condition(args[counter].to_string(), args[counter + 2].to_string(), ConditionOperator::MinorEqual))            
                } else if args[counter + 1].eq("=") {
                    result.where_condition = Some(build_condition(args[counter].to_string(), args[counter + 2].to_string(), ConditionOperator::Equal))            
                } else if args[counter + 1].eq(">=") {
                    result.where_condition = Some(build_condition(args[counter].to_string(), args[counter + 2].to_string(), ConditionOperator::HigherEqual))            
                } else if args[counter + 1].eq(">") {
                    result.where_condition = Some(build_condition(args[counter].to_string(), args[counter + 2].to_string(), ConditionOperator::Higher))            
                } else {
                    return Err(error::WHERE_MAL_FORMATEADO)
                }
            }

            if counter < args.len() {  
                if args[counter].eq("ORDER") && args[counter + 1].eq("BY") {
                    if counter + 3 == args.len() {                                          // ORDER BY column
                        result.order_by = Some((args[counter + 2].to_string(), true));
                    } else if counter + 4 == args.len() {                                   // ORDER BY column ASC/DESC
                        if args[counter + 3].eq("ASC") || args[counter + 3].eq("DESC") {
                            result.order_by = Some((args[counter + 2].to_string(), args[counter + 2].eq("ASC")));
                        } else {
                            return Err(error::ORDER_BY_MAL_FORMATEADO)
                        }
                    } else {
                        return Err(error::ORDER_BY_MAL_FORMATEADO)
                    }
                } else {
                    return Err(error::ORDER_BY_MAL_FORMATEADO)
                }
            }
            Ok(0)        
        }
    }
}

fn check_operation_format(args: &Vec<String>) -> Result<QueryType, u32> {
    // Check correct operation sintaxis
    let operation = &args[0];
    if operation.eq("DELETE") {
        check_delete_format(args)
    } else if operation.eq("INSERT") {
        check_insert_format(args)
    } else if operation.eq("SELECT") {
        check_select_format(args)
    } else if operation.eq("UPDATE") {
        check_update_format(args)
    } else {
        Err(error::OPERACION_INVALIDA)          
    }
}

fn check_delete_format(args: &Vec<String>) -> Result<QueryType, u32> {
    let non_valid_keywords = vec!["INSERT","INTO","VALUES","SELECT","ORDER","BY","UPDATE","SET"];
    if args[1].eq("FROM") && args[3].eq("WHERE") && check_non_valid_keywords(args, non_valid_keywords) {
        Ok(QueryType::DELETE)       
    } else {
        Err(error::DELETE_MAL_FORMATEADO)          
    }
}

fn check_insert_format(args: &Vec<String>) -> Result<QueryType, u32> {
    let non_valid_keywords = vec!["DELETE","FROM","SELECT","ORDER","BY","UPDATE","SET","WHERE","AND","OR","NOT"];
    if args[1].eq("INTO") && check_non_valid_keywords(args, non_valid_keywords){
        if (args.len() - 3) % 2 != 0 || args.len() - 3 < 0 {
            Err(2)
        } else{
            let mut counter = 0;
            let correct_value_len = (args.len() - 3)/2;    // INSERT INTO a b c VALUES x y z. Len = 9. Correct len = 3.          
            while counter < correct_value_len && ! args[counter].eq("VALUES"){
                counter += 1;
            } 
    
            if counter == correct_value_len && args[counter].eq("VALUES") {
                Ok(QueryType::INSERT)
            } else{
                Err(error::INSERT_MAL_FORMATEADO)      
            }    
        }
    }
    else {
        Err(error::INSERT_MAL_FORMATEADO)              
    }
}

fn check_select_format(args: &Vec<String>) -> Result<QueryType, u32> {
    let non_valid_keywords = vec!["DELETE","INSERT","INTO","VALUES","UPDATE","SET"];
    if check_non_valid_keywords(args, non_valid_keywords) {
        // TODO format select 
        Ok(QueryType::SELECT)
    } else {
        Err(error::SELECT_MAL_FORMATEADO)
    }
}

fn check_update_format(args: &Vec<String>) -> Result<QueryType, u32> {
    let non_valid_keywords = vec!["DELETE","FROM","INSERT","INTO","SELECT","VALUES","ORDER","BY"];
    if args[2].eq("SET") && args.contains(&"WHERE".to_string()) && check_non_valid_keywords(args, non_valid_keywords) {
        Ok(QueryType::UPDATE)
    } else{
        Err(error::UPDATE_MAL_FORMATEADO)
    }
}

fn check_non_valid_keywords(args: &Vec<String>, non_valid_keywords:  Vec<&str>) -> bool {
    // Checks if args contains any non valid keyword
    let mut result = true;
    let mut counter = 0;

    while counter < args.len() && result {
        if non_valid_keywords.contains(&&args[counter].as_str()) {
            result = false;
        } else {
            counter += 1;
        }
    }

    result
}

fn validate_query(query: Query) -> Result<Query,u32> {
    // Pre: Sintactical query OK 
    // Post: Valid query for execution
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(_) => {},
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO) 
        },
        None => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)    
    }

    match &query.operation {
        Some(op) => {
            match op {
                QueryType::DELETE => validate_delete_query(query),
                QueryType::INSERT => validate_insert_query(query),
                QueryType::SELECT => validate_select_query(query),
                QueryType::UPDATE => validate_update_query(query),
            }
        },
        None => return Err(error::OPERACION_INVALIDA)
    }
}

fn validate_delete_query(query: Query) -> Result<Query,u32> {
    if query.columns.is_none() && query.values.is_none() && query.where_condition.is_some() {
        match &query.table {
            Some(table) => {
                match File::open(table) {
                    Ok(file) => {
                        match get_columns(file) {
                            Some(columns) => check_columns_contains_condition(columns,query),
                            None => return Err(error::ARCHIVO_VACIO)
                        }
                    },
                    Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)    
                }
            },
            None => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)
        }
    } else {
        Err(error::DELETE_MAL_FORMATEADO)
    }
}

fn validate_insert_query(query: Query) -> Result<Query,u32> {
    // TODO
    Ok(query)
}

fn validate_select_query(query: Query) -> Result<Query,u32> {
    // TODO 
    Ok(query)
}

fn validate_update_query(query: Query) -> Result<Query,u32> {
    // TODO 
    Ok(query)
}

fn get_columns(file: File) -> Option<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(_) => {
            let mut split = line.split(',');
            let mut element_opt = split.next();
            while element_opt.is_some() {
                match element_opt {
                    Some(x) => result.push(x.to_string()),
                    None => continue
                }
                element_opt = split.next();
            }
            
            Some(result)
        },
        Err(_) => None
    }
}

fn check_columns_contains_condition(columns: Vec<String>, query: Query) -> Result<Query,u32> {
    // TODO 
    match get_where_columns(&query) {
        Some(column) => {
            if columns.contains(&column) {
                Ok(query)                        
            } else {
                Err(error::ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS)
            }
        }, 
        None => return Err(error::NO_WHERE)
    }
} 

fn get_where_columns(query: &Query) -> Option<String> {
    match &query.where_condition {
        Some(cond) => {
            match &cond.column {
                Some(col) => return Some(col.to_string()),
                None => return None
            }
        },
        None => return None
    }
}

fn text_to_vec(text_query: &String) -> Vec<String> {
    // Text to vector. Tokenization by space, new line & coma
    let mut tmp_text_query = text_query.to_string();        
    tmp_text_query = tmp_text_query.replace("\n", " ");
    tmp_text_query = tmp_text_query.replace(",", "");
    tmp_text_query = tmp_text_query.replace(";", "");
    let mut split = tmp_text_query.split(' ');

    let mut result: Vec<String> = Vec::new();
    let mut element_opt = split.next();
    while element_opt.is_some() {
        match element_opt {
            Some(x) => {
                result.push(x.to_string());
            },
            None => continue    
        }

        element_opt = split.next();
    }
    result
}

fn vec_to_query(args: &Vec<String>, path: &String) -> Result<Query,u32>{
    // Vec to non validated query 
    let mut result: Query = build_empty_query(); 
    match check_operation_format(&args) {       
        Ok(x) => {
            match &x {
                QueryType::DELETE => match separate_args_delete(args, path, &mut result){
                    Ok(_) => result.operation = Some(x),
                    Err(x) => return Err(x)
                }, 
                QueryType::INSERT => match separate_args_insert(args, path, &mut result) {
                    Ok(_) => result.operation = Some(x),
                    Err(x) => return Err(x)
                },
                QueryType::SELECT => match separate_args_select(args, path, &mut result) {
                    Ok(_) => result.operation = Some(x),
                    Err(x) => return Err(x)
                },
                QueryType::UPDATE => match separate_args_update(args, path, &mut result) {
                    Ok(_) => result.operation = Some(x),
                    Err(x) => return Err(x)
                }
            }
        },
        Err(x) => return Err(x)
    } 

    Ok(result)
}

pub fn get_file_first_line(query: &Query) -> Option<String> {
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(f) => {
                let mut reader: BufReader<File> = BufReader::new(f);
                let mut line = String::new();

                match reader.read_line(&mut line) {
                    Ok(_) => Some(line),
                    Err(_) => None
                }
            },
            Err(_) => None
        },
        None => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_text_to_vec1(){
        let rt1 = text_to_vec(&String::from("SELECT * FROM table"));
        assert_eq!(rt1, vec!["SELECT","*","FROM","table"]);
    }

    #[test] 
    fn test_text_to_vec2(){
        let rt2 = text_to_vec(&String::from("SELECT id, producto, id_cliente\nFROM ordenes\nWHERE cantidad > 1"));
        assert_eq!(rt2, vec!["SELECT","id","producto","id_cliente","FROM","ordenes","WHERE","cantidad",">","1"]);
    }


}

/*
fn check_where_format(args: &Vec<String>, start_position: usize) -> bool {
    let mut result = true;

    let mut counter: usize   = start_position;

    let mut not_detected: bool = false;
    let mut op_detected: bool = false;

    while counter < args.len() && result {
        if args[counter].eq("NOT") {
            if not_detected || op_detected {
                result = false;  // NOT NOT. Que estas haciendo ?
            } else {
                not_detected = true;
                counter += 1;
            }
        } else if args[counter].eq("AND") || args[counter].eq("OR")  {
            if op_detected {
                result = false;  // AND OR , OR AND, OR OR, AND AND. Que estas haciendo ? 
            } else {
                op_detected = true;
                counter += 1;
            }
        } else if counter + 3 > args.len() {
            result = false;
        } else {

        }
    }

    result
}

fn check_table_exist(path: &String, args: &Vec<String>, operation: &QueryType) -> Result<String, u32>{
    // Asume query bien formateado 
    let mut table= String::from(path);
    table.push('/');
    match &operation {
        QueryType::DELETE => table.push_str(&args[2]),
        QueryType::INSERT => table.push_str(&args[2]),
        QueryType::SELECT => {  
            // Debo encontrar cual es la tabla (asumiendo query bien formateado) 
            let mut counter = 2;
            while ! args[counter].eq("FROM"){ 
                counter += 1;
            }
            table.push_str(&args[counter + 1])
        },
        QueryType::UPDATE => table.push_str(&args[1]),
    }
    
    match File::open(&table) { // TODO: Path::exist()?
        Ok(_) => return Ok(table),  
        Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)    
    }
}
*/