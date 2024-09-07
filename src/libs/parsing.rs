use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::libs::error;
use crate::query::Query;
use crate::query::query_type::QueryType;
use crate::query::build_empty_query;

pub fn build_query(text_query: &String, path: &String) -> Result<Query, u32> {
    // Full "Compilation" process of query. Tokenization, sintactic analysis, semantic and build 
    let args = text_to_vec(&text_query);  
    match vec_to_query(&args, path) {               // sintactic analysis
        Ok(x) => return validate_query(x),   // Semantic analysis
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

fn separate_args_delete(args: &Vec<String>, path: &String, result: &mut Query) {  
    result.table = Some(merge_table_and_path(path, &args[2]));   

    let mut where_condition: Vec<String> = Vec::new();
    where_condition.push(args[4].to_string());

    let mut counter = 5; 
    while counter < args.len() {
        where_condition.push(args[counter].to_string());
        counter += 1;
    }

    result.where_condition = Some(where_condition);
}

fn separate_args_insert(args: &Vec<String>, path: &String, result: &mut Query) {
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
}

fn separate_args_update(args: &Vec<String>, path: &String, result: &mut Query) {
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
    counter += 1;

    let mut where_condition = Vec::new();
    while counter < args.len() {
        where_condition.push(args[counter].to_string());
        counter += 1;
    }

    result.columns = Some(columns);
    result.where_condition = Some(where_condition);
    result.values = Some(values);
}

fn separate_args_select(args: &Vec<String>, path: &String, result: &mut Query) { 
    let mut columns: Vec<String> = Vec::new();
    let mut counter = 1; 
    while counter < args.len() && !args[counter].eq("FROM") {
        columns.push(args[counter].to_string());
        counter += 1;
    }
    counter += 1;
    result.table = Some(merge_table_and_path(path, &args[counter]));
    
    let mut where_condition = Vec::new();
    while counter < args.len() && !args[counter].eq("ORDER") {
        where_condition.push(args[counter].to_string());
        counter += 1;
    }
    result.where_condition = Some(where_condition);

    // Last condition returns false on desc, true on asc 
    result.order_by = Some((args[counter + 2].to_string(),counter + 3 != args.len()));
}

fn check_operation_format(args: &Vec<String>) -> Result<QueryType, u32> {
    // Check correct operation sintaxis
    let operation = &args[0];
    if operation.eq("DELETE") {
        check_delete_format(args)
    } else if operation.eq("INSERT" ) {
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
    if args[1].eq("FROM") && args[3].eq("WHERE"){
        Ok(QueryType::DELETE)            
    } else {
        Err(error::DELETE_MAL_FORMATEADO)          
    }
}

fn check_insert_format(args: &Vec<String>) -> Result<QueryType, u32> {
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
    // TODO format select 
    Ok(QueryType::SELECT)
}

fn check_update_format(args: &Vec<String>) -> Result<QueryType, u32> {
    // TODO format select 
    Ok(QueryType::UPDATE)
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


fn validate_query(query: Query) -> Result<Query,u32> {
    // Pre: Sintactical query OK 
    // Post: Valid query for execution
    match File::open(&query.table.unwrap()) { 
        Ok(_) => {},  
        Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)    
    }

    match &query.operation.unwrap() {
        QueryType::DELETE => validate_delete_query(&query),
        QueryType::INSERT => validate_insert_query(query),
        QueryType::SELECT => validate_select_query(query),
        QueryType::UPDATE => validate_update_query(query),
    }
}

fn validate_delete_query(query: &Query) -> Result<Query,u32> {
    if query.columns.is_none() && query.values.is_none() && query.columns.is_none() {
        match query.table {
            Some(table) => {

            },
            Err(_) => Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)
        }
        
        match File::open(query.table.unwrap()) { 
            Ok(x) => {
                match get_columns(x) {
                    Some(x) => check_columns_contains(x, &query),
                    None => return Err(error::ARCHIVO_SIN_COLUMNAS)
                }                
            },  
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO)    
        }
    } else {
        Err(error::DELETE_MAL_FORMATEADO)
    }
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

fn check_columns_contains(columns: Vec<String>, query: &Query) -> Result<Query,u32> {

} 


fn text_to_vec(text_query: &String) -> Vec<String> {
    // Text to vector. Tokenization by space, new line & coma
    let mut result: Vec<String> = Vec::new();
    let mut split = text_query.split(&[' ', '\n']);

    let mut element_opt = split.next();
    while element_opt.is_some() {
        match element_opt {
            Some(x) => {
                if x.contains(',') {
                    let mut tmp = String::from(x);
                    tmp.pop();
                    result.push(tmp);
                }
                else {
                    result.push(x.to_string());
                }
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
                QueryType::DELETE => separate_args_delete(args, path, &mut result), 
                QueryType::INSERT => separate_args_insert(args, path, &mut result),
                QueryType::SELECT => separate_args_select(args, path, &mut result),
                QueryType::UPDATE => separate_args_update(args, path, &mut result)
            }
            result.operation = Some(x);        
        },
        Err(x) => return Err(x)
    } 

    Ok(result)
}

/* 
fn build_query(text_query: &String, path: &String) -> Result<Query, u32>{
    // Crea query valida o devuelve codigo de error.
    let args = text_to_vec(text_query);
    let mut resultado = build_empty_query();

    match full_check_query(&args, &path, &mut resultado) {
        Ok(_) => return Ok(resultado), 
        Err(x) => return Err(x)
    }

}

fn check_columns_exist(args: &Vec<String>, result: &mut Query) -> Result<Vec<String>,u32> {
    // TODO columns exist 
    let file = File::open(&result.table.unwrap() );
    match file {
        Ok(_) => {
            let mut reader = BufReader::new(file);

            let mut line = String::new();
            match reader.read_line(&mut line) {
                
            }
        }
        Err(_) => return Err(5)
    }

}

fn text_to_vec(text_query: &String) -> Vec<String> {
    let mut resultado: Vec<String> = Vec::new();
    let mut split = text_query.split(' ');

    let mut element_opt = split.next();
    while element_opt.is_some() {
        match element_opt {
            Some(x) => {

                if x.contains('\n') {
                    let mut split2 = x.split('\n');
                    let mut element_split2 = split2.next();
                    resultado.push(element_split2.unwrap().to_string());
                    element_split2 = split2.next();
                    resultado.push(element_split2.unwrap().to_string());
                } else if x.contains(',') {

                } else {
                    resultado.push(x.to_string())
                }
            },    // TODO: Check si esto esta bien 
            None => continue
        }
       element_opt = split.next();
    }

    resultado
}

fn full_check_query(args: &Vec<String>, path: &String) -> Result<Query, u32> {
    let mut error_code :u32 = 0;
    let mut result = build_empty_query();
    match check_operation(&args) {
        Ok(x) => result.operation = Some(x),
        Err(x) => return Err(x)
    } 
    match check_table_exist(path, args, &result.operation.unwrap()){
        Ok(x) => result.table = Some(x),
        Err(x) => return Err(x)
    } 
    match check_columns_exist(args,&result.table.unwrap()) {

    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_text_to_vec(){
        let rt1 = text_to_vec(&String::from("SELECT * FROM table"));
        assert_eq!(rt1, vec!["SELECT","*","FROM","table"]);
        let rt2 = text_to_vec(&String::from("SELECT id, producto, id_cliente\nFROM ordenes\nWHERE cantidad > 1"));
        assert_eq!(rt2, vec!["SELECT","id","producto","id_cliente","FROM","ordenes","WHERE","cantidad>1"]);
    }
}