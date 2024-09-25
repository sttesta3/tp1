use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::condition::{build_condition, build_not_condition, Condition};
//use crate::condition::complex_condition::{add_node_to_tree, build_complex_condition, build_simple_condition, ComplexCondition, tree_check, check_precedence};
use crate::condition::condition_type::ConditionOperator;
//use crate::condition::{add_node_to_tree, build_complex_condition, build_condition, build_empty_complex_condition, get_where_columns, tree_check, ComplexCondition, Condition};
// use crate::condition::Condition;
use crate::libs::error;
use crate::query::build_empty_query;
use crate::query::query_type::QueryType;
use crate::query::{Query, DELETE_MIN_LEN, INSERT_MIN_LEN, SELECT_MIN_LEN, UPDATE_MIN_LEN};

use super::error::{DELETE_MAL_FORMATEADO, ORDER_BY_MAL_FORMATEADO, WHERE_MAL_FORMATEADO};

/// Build query for execution
pub fn build_query(text_query: &String, path: &String) -> Result<Query, u32> {
    let args = text_to_vec(text_query, false);
    match vec_to_query(&args, path) {
        Ok(x) => validate_query(x),
        Err(x) => Err(x),
    }
}

/// Merge table and path. Aux funcion
///
/// ```
/// let result = merge_table_and_path(String::from("tablas"), "clientes.csv");
/// assert!(result,String::from("tablas/clientes.csv"));
/// ```
fn merge_table_and_path(path: &String, table: &str) -> String {
    let mut table_full: String = path.to_string();
    table_full.push('/');
    table_full.push_str(table);
    table_full.push_str(".csv");

    table_full
}

/// Parse args from vec to query (special case for delete)
fn parse_args_delete(args: &[String], path: &String, result: &mut Query) -> Result<u32, u32> {
    if args.len() < DELETE_MIN_LEN {
        Err(error::DELETE_MAL_FORMATEADO)
    } else {
        match File::open(merge_table_and_path(path, &args[2])) {
            Ok(_) => result.table = Some(merge_table_and_path(path, &args[2])),
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        }

        if args.len() != DELETE_MIN_LEN {
            return Err(error::DELETE_MAL_FORMATEADO);
        } else if args[3].eq("WHERE") {
            let mut counter: usize = 4;
            match parse_where_condition(result, args, &mut counter) {
                Ok(cond) => result.where_condition = Some(cond),
                Err(x) => return Err(x),
            }
        } else {
            return Err(DELETE_MAL_FORMATEADO);
        }

        Ok(0)
    }
}

/// Parse args from vec to query (special case for insert)
fn parse_args_insert(args: &[String], path: &String, result: &mut Query) -> Result<u32, u32> {
    if args.len() < INSERT_MIN_LEN {
        Err(error::INSERT_MAL_FORMATEADO)
    } else {
        match File::open(merge_table_and_path(path, &args[2])) {
            Ok(_) => result.table = Some(merge_table_and_path(path, &args[2])),
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        }

        let mut string_columns: Vec<String> = Vec::new();
        let mut counter = 3;
        while counter < args.len() && !args[counter].eq("VALUES") {
            string_columns.push(args[counter].to_string());
            counter += 1;
        }

        counter += 1;
        let mut values: Vec<String> = Vec::new();
        while counter < args.len() {
            values.push(args[counter].to_string());
            counter += 1;
        }

        match get_columns_position(result, &string_columns) {
            Ok(x) => result.columns = Some(x),
            Err(x) => return Err(x),
        }
        result.values = Some(values);

        Ok(0)
    }
}

/// Parse args from vec to query (special case for update)
fn parse_args_update(args: &[String], path: &String, result: &mut Query) -> Result<u32, u32> {
    if args.len() < UPDATE_MIN_LEN {
        Err(error::UPDATE_MAL_FORMATEADO)
    } else {
        match File::open(merge_table_and_path(path, &args[1])) {
            Ok(_) => result.table = Some(merge_table_and_path(path, &args[1])),
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        }

        let mut counter: usize = 3;
        let mut string_columns: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        while counter < args.len() && !args[counter].eq("WHERE") {
            if counter % 3 == 0 {
                string_columns.push(args[counter].to_string());
            } else if (counter % 3) == 2 {
                values.push(args[counter].to_string());
            }
            counter += 1;
        }

        match get_columns_position(result, &string_columns) {
            Ok(x) => result.columns = Some(x),
            Err(x) => return Err(x),
        }
        result.values = Some(values);

        match parse_where_condition(result, args, &mut counter) {
            Ok(cond) => result.where_condition = Some(cond),
            Err(x) => return Err(x),
        }

        Ok(0)
    }
}

/// Parse args from vec to query (special case for select)
fn parse_args_select(args: &[String], path: &String, result: &mut Query) -> Result<u32, u32> {
    if args.len() < SELECT_MIN_LEN {
        Err(error::SELECT_MAL_FORMATEADO)
    } else {
        let mut string_columns: Vec<String> = Vec::new();
        let mut counter = 1;
        while counter < args.len() && !args[counter].eq("FROM") {
            string_columns.push(args[counter].to_string());
            counter += 1;
        }
        counter += 1;

        match File::open(merge_table_and_path(path, &args[counter])) {
            Ok(_) => result.table = Some(merge_table_and_path(path, &args[counter])),
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        }

        if !&string_columns[0].eq("*") {
            match get_columns_position(result, &string_columns) {
                Ok(x) => result.columns = Some(x),
                Err(x) => return Err(x),
            }
        }

        counter += 1;

        if counter == args.len() {
            return Ok(0);
        } else if args[counter].eq("WHERE") {
            counter += 1;
            match parse_where_condition(result, args, &mut counter) {
                Ok(cond) => result.where_condition = Some(cond),
                Err(x) => return Err(x),
            }
        }

        if counter == args.len() {
            Ok(0)
        } else {
            match parse_order_by(args, &mut counter) {
                Ok(cond) => {
                    result.order_by = Some(cond);
                    Ok(0)
                }
                Err(x) => Err(x),
            }
        }
    }
}

/// Aux Function. parse from string columns to the index of the columns in the table
/// All columns should be in the table  
fn get_columns_position(query: &Query, string_cols: &[String]) -> Result<Vec<usize>, u32> {
    match get_file_first_line(query) {
        Some(line) => {
            let mut result: Vec<usize> = Vec::new();
            let columns = text_to_vec(&line, true);

            let mut counter = 0;
            while counter < string_cols.len() {
                if columns.contains(&string_cols[counter]) {
                    match find_column_position(&string_cols[counter], &columns) {
                        Ok(x) => {
                            result.push(x);
                            counter += 1;
                        }
                        Err(x) => return Err(x),
                    }
                } else {
                    return Err(error::ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS);
                }
            }
            Ok(result)
        }
        None => Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
    }
}

/// Parse order by
///
/// Pre: Counter == 3 or counter == 4
fn parse_order_by(args: &[String], counter: &mut usize) -> Result<(String, bool), u32> {
    if *counter + 3 == args.len() || *counter + 4 == args.len() {
        if args[*counter].eq("ORDER") && args[*counter + 1].eq("BY") {
            let column = args[*counter + 2].to_string();
            let mut asc = true;
            if *counter + 4 == args.len() {
                if args[*counter + 3].eq("DESC") {
                    asc = false;
                } else if !args[*counter + 3].eq("ASC") {
                    return Err(ORDER_BY_MAL_FORMATEADO);
                }
            }
            Ok((column, asc))
        } else {
            Err(ORDER_BY_MAL_FORMATEADO)
        }
    } else {
        Err(ORDER_BY_MAL_FORMATEADO)
    }
}

/// Parse boolean where condition, advancing the counter
///
/// Pre:  Query args and counter in next position to WHERE
/// Post: Complex condition for query and counter at the next position of the last element of condition
///
/// EXAMPLE:
/// Pre:  parse_where_condition(query, vec!["SELECT","*","FROM","tabla","WHERE","id","<","5","ORDER","BY","id"],5);
/// Post: Result<parsed_condition>, counter =  8
fn parse_where_condition(
    query: &Query,
    args: &[String],
    counter: &mut usize,
) -> Result<Vec<Vec<Condition>>, u32> {
    if args[*counter].eq("AND") || args[*counter].eq("OR") {
        Err(WHERE_MAL_FORMATEADO)
    } else {
        // Parse into boolean vector
        let mut result: Vec<Vec<Condition>> = Vec::new();
        result.push(Vec::new());
        while *counter < args.len() && !args[*counter].eq("ORDER") {
            match parse_next_condition(query, args, counter, &mut result) {
                Ok(_) => continue,
                Err(x) => return Err(x),
            }
        }

        // Check valid result
        if check_valid_bool(&result) {
            Ok(result)
        } else {
            Err(WHERE_MAL_FORMATEADO)
        }
    }
}

/// Pre: Query, args, position counter and wip vector
/// Post: Parses the next simple condition into boolean vector
fn parse_next_condition(
    query: &Query,
    args: &[String],
    counter: &mut usize,
    vec: &mut Vec<Vec<Condition>>,
) -> Result<u32, u32> {
    if *counter + 2 >= args.len() {
        Err(WHERE_MAL_FORMATEADO)
    } else {
        if args[*counter + 1].eq("AND") || args[*counter + 1].eq("OR") {
            return Err(WHERE_MAL_FORMATEADO);
        } else if args[*counter].eq("OR") {
            vec.push(Vec::new());
            *counter += 1;
        } else if args[*counter].eq("AND") {
            *counter += 1;
        } else if args[*counter].eq("NOT") {
            let position = vec.len() - 1;
            vec[position].push(build_not_condition()); // String vacio
            *counter += 1;
        } else {
            match get_columns_position(query, &[args[*counter].to_string()]) {
                Err(x) => return Err(x),
                Ok(x) => {
                    let simple_condition: Condition = if args[*counter + 1].eq("<") {
                        build_condition(
                            x[0],
                            args[*counter + 2].to_string(),
                            ConditionOperator::Minor,
                        )
                    } else if args[*counter + 1].eq("<=") {
                        build_condition(
                            x[0],
                            args[*counter + 2].to_string(),
                            ConditionOperator::MinorEqual,
                        )
                    } else if args[*counter + 1].eq("=") {
                        build_condition(
                            x[0],
                            args[*counter + 2].to_string(),
                            ConditionOperator::Equal,
                        )
                    } else if args[*counter + 1].eq(">=") {
                        build_condition(
                            x[0],
                            args[*counter + 2].to_string(),
                            ConditionOperator::HigherEqual,
                        )
                    } else if args[*counter + 1].eq(">") {
                        build_condition(
                            x[0],
                            args[*counter + 2].to_string(),
                            ConditionOperator::Higher,
                        )
                    } else {
                        return Err(WHERE_MAL_FORMATEADO);
                    };
                    let position = vec.len() - 1;
                    vec[position].push(simple_condition);
                    *counter += 3;
                }
            }
        }
        Ok(0)
    }
}

// Pre: Boolean vector (parsed boolean condition)
// Post: Bool for valid or invalid result
fn check_valid_bool(boolean_expresion: &[Vec<Condition>]) -> bool {
    let mut valid = true;
    let mut counter = 0;
    let mut sub_counter;
    let mut open_not = false;
    while counter < boolean_expresion.len() && valid {
        valid = boolean_expresion[counter].is_empty();

        sub_counter = 0;
        while sub_counter < boolean_expresion[counter].len() && valid {
            open_not = !(open_not && boolean_expresion[counter][sub_counter].value.is_some());
            sub_counter += 1;
        }

        valid = !open_not;
        counter += 1;
    }

    valid
}

/// find column position
fn find_column_position(column_name: &String, columns: &[String]) -> Result<usize, u32> {
    let mut counter = 0;
    while counter < columns.len() {
        if column_name.eq(&columns[counter]) {
            return Ok(counter);
        } else {
            counter += 1;
        }
    }
    Err(error::ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS)
}

/// Check correct operation sintaxis
fn check_operation_format(args: &[String]) -> Result<QueryType, u32> {
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

/// Check correct operation sintaxis (special case for delete)
fn check_delete_format(args: &[String]) -> Result<QueryType, u32> {
    let non_valid_keywords = vec![
        "INSERT", "INTO", "VALUES", "SELECT", "ORDER", "BY", "UPDATE", "SET",
    ];
    if args[1].eq("FROM")
        && args[3].eq("WHERE")
        && check_non_valid_keywords(args, non_valid_keywords)
    {
        Ok(QueryType::DELETE)
    } else {
        Err(error::DELETE_MAL_FORMATEADO)
    }
}

/// Check correct operation sintaxis (special case for insert)
fn check_insert_format(args: &[String]) -> Result<QueryType, u32> {
    let non_valid_keywords = vec![
        "DELETE", "FROM", "SELECT", "ORDER", "BY", "UPDATE", "SET", "WHERE", "AND", "OR", "NOT",
    ];
    if args[1].eq("INTO") && check_non_valid_keywords(args, non_valid_keywords) {
        if (args.len() - 4) % 2 != 0 {
            Err(2)
        } else {
            let mut counter = 0;
            let correct_value_len = (args.len() - 4) / 2; // INSERT INTO tabla a b c VALUES x y z. Len = 10. Correct len = 3.
            while counter < correct_value_len && !args[3 + counter].eq("VALUES") {
                counter += 1;
            }

            if counter == correct_value_len && args[3 + counter].eq("VALUES") {
                Ok(QueryType::INSERT)
            } else {
                Err(error::INSERT_MAL_FORMATEADO)
            }
        }
    } else {
        Err(error::INSERT_MAL_FORMATEADO)
    }
}

/// Check correct operation sintaxis (special case for select)
fn check_select_format(args: &[String]) -> Result<QueryType, u32> {
    let non_valid_keywords = vec!["DELETE", "INSERT", "INTO", "VALUES", "UPDATE", "SET"];
    if check_non_valid_keywords(args, non_valid_keywords) {
        // TODO format select
        Ok(QueryType::SELECT)
    } else {
        Err(error::SELECT_MAL_FORMATEADO)
    }
}

/// Check correct operation sintaxis (special case for update)
fn check_update_format(args: &[String]) -> Result<QueryType, u32> {
    let non_valid_keywords = vec![
        "DELETE", "FROM", "INSERT", "INTO", "SELECT", "VALUES", "ORDER", "BY",
    ];
    if args[2].eq("SET")
        && args.contains(&"WHERE".to_string())
        && check_non_valid_keywords(args, non_valid_keywords)
    {
        Ok(QueryType::UPDATE)
    } else {
        Err(error::UPDATE_MAL_FORMATEADO)
    }
}

/// Check if args for query contains non valid keywords
fn check_non_valid_keywords(args: &[String], non_valid_keywords: Vec<&str>) -> bool {
    let mut result = true;
    let mut counter = 0;

    while counter < args.len() && result {
        if non_valid_keywords.contains(&args[counter].as_str()) {
            result = false;
        } else {
            counter += 1;
        }
    }

    result
}

/// Pre: Sintactical query OK
/// Post: Valid query for execution
fn validate_query(query: Query) -> Result<Query, u32> {
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(_) => {}
            Err(_) => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        },
        None => return Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
    }

    match &query.operation {
        Some(op) => match op {
            QueryType::DELETE => validate_delete_query(query),
            QueryType::INSERT => validate_insert_query(query),
            QueryType::SELECT => validate_select_query(query),
            QueryType::UPDATE => validate_update_query(query),
        },
        None => Err(error::OPERACION_INVALIDA),
    }
}

/// Validate query: Special case for delete
fn validate_delete_query(query: Query) -> Result<Query, u32> {
    if query.columns.is_none() && query.values.is_none() && query.where_condition.is_some() {
        validate_table(query)
    } else {
        Err(error::DELETE_MAL_FORMATEADO)
    }
}

/// Validate query: Special case for insert
fn validate_insert_query(query: Query) -> Result<Query, u32> {
    if query.values.is_some() && query.columns.is_some() && query.where_condition.is_none() {
        validate_table(query)
    } else {
        Err(error::INSERT_MAL_FORMATEADO)
    }
}

/// Validate query: Special case for select
fn validate_select_query(query: Query) -> Result<Query, u32> {
    if query.values.is_none() {
        validate_table(query)
    } else {
        Err(error::SELECT_MAL_FORMATEADO)
    }
}

/// Validate query: Special case for update
fn validate_update_query(query: Query) -> Result<Query, u32> {
    if query.values.is_some() && query.columns.is_some() && query.where_condition.is_some() {
        validate_table(query)
    } else {
        Err(error::UPDATE_MAL_FORMATEADO)
    }
}

/// Validate if table is valid
fn validate_table(query: Query) -> Result<Query, u32> {
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(file) => match get_columns(file) {
                Some(_) => Ok(query),
                None => Err(error::ARCHIVO_VACIO),
            },
            Err(_) => Err(error::ARCHIVO_NO_PUDO_SER_ABIERTO),
        },
        None => Err(error::FALTA_ARCHIVO),
    }
}

/// AUX: Get columns from file
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
                    None => continue,
                }
                element_opt = split.next();
            }

            Some(result)
        }
        Err(_) => None,
    }
}

/// AUX: Get first line from file
/// Pre: query
/// Post: File first line, if any
pub fn get_file_first_line(query: &Query) -> Option<String> {
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(f) => {
                let mut reader: BufReader<File> = BufReader::new(f);
                let mut line = String::new();

                match reader.read_line(&mut line) {
                    Ok(_) => {
                        line = line.replace('\n', "");
                        Some(line)
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        },
        None => None,
    }
}

/// Tokenization of text query by space, new line & coma
pub fn text_to_vec(text_query: &String, coma: bool) -> Vec<String> {
    let mut tmp_text_query = text_query.to_string();
    tmp_text_query = tmp_text_query.replace('\n', " ");
    if !coma {
        tmp_text_query = tmp_text_query.replace(',', "");
    } else {
        tmp_text_query = tmp_text_query.replace(',', " ");
    }
    tmp_text_query = tmp_text_query.replace(';', "");
    let mut split = tmp_text_query.split(' ');

    let mut result: Vec<String> = Vec::new();
    let mut element_opt = split.next();
    while element_opt.is_some() {
        match element_opt {
            Some(x) => {
                result.push(x.to_string());
            }
            None => continue,
        }

        element_opt = split.next();
    }
    result
}

/// Parses vector of tokens into query
/// Pre:  Vec of tokens from query & path to tables
/// Post: Vec to non validated query
fn vec_to_query(args: &[String], path: &String) -> Result<Query, u32> {
    let mut result: Query = build_empty_query();
    match check_operation_format(args) {
        Ok(x) => match &x {
            QueryType::DELETE => match parse_args_delete(args, path, &mut result) {
                Ok(_) => result.operation = Some(x),
                Err(x) => return Err(x),
            },
            QueryType::INSERT => match parse_args_insert(args, path, &mut result) {
                Ok(_) => result.operation = Some(x),
                Err(x) => return Err(x),
            },
            QueryType::SELECT => match parse_args_select(args, path, &mut result) {
                Ok(_) => result.operation = Some(x),
                Err(x) => return Err(x),
            },
            QueryType::UPDATE => match parse_args_update(args, path, &mut result) {
                Ok(_) => result.operation = Some(x),
                Err(x) => return Err(x),
            },
        },
        Err(x) => return Err(x),
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_vec1() {
        let rt1 = text_to_vec(&String::from("SELECT * FROM table"), false);
        assert_eq!(rt1, vec!["SELECT", "*", "FROM", "table"]);
    }

    #[test]
    fn test_text_to_vec2() {
        let rt2 = text_to_vec(
            &String::from("SELECT id, producto, id_cliente\nFROM ordenes\nWHERE cantidad > 1"),
            false,
        );
        assert_eq!(
            rt2,
            vec![
                "SELECT",
                "id",
                "producto",
                "id_cliente",
                "FROM",
                "ordenes",
                "WHERE",
                "cantidad",
                ">",
                "1"
            ]
        );
    }
}

/*
fn check_columns_contains_condition(columns: Vec<String>, query: Query) -> Result<Query, u32> {
    // TODO
    match get_where_columns(&query) {
        Some(column) => {
            if columns.contains(&column) {
                Ok(query)
            } else {
                Err(error::ARCHIVO_NO_CONTIENE_COLUMNAS_SOLICITADAS)
            }
        }
        None => Err(error::NO_WHERE),
    }
}

fn get_where_columns(query: &Query) -> Option<String> {
    match &query.where_condition {
        Some(cond) => cond.column.as_ref().map(|col| col.to_string()),
        None => None,
    }
}

fn get_condition_node(query: &Query, args: &[String], counter: &mut usize) -> Result<ComplexCondition, u32> {
    // Pre:  Query args and counter of args
    // Post: New node && increments counter
    let new_node: ComplexCondition;
    if args[*counter].eq("AND") {
        new_node = build_complex_condition(BooleanOperator::AND, None, None);
        *counter += 1;
    } else if args[*counter].eq("OR") {
        new_node = build_complex_condition(BooleanOperator::OR, None, None);
        *counter += 1;
    } else if args[*counter].eq("NOT") {
        new_node = build_complex_condition(BooleanOperator::NOT, None, None);
        *counter += 1;
    } else if *counter + 3 < args.len() {
        match get_columns_position(query, &[args[*counter].to_string()]) {
            Err(x) => return Err(x),
            Ok(x) => {
                let simple_condition: Condition = if args[*counter + 2].eq("<") {
                    build_condition(x[0], args[*counter + 2].to_string(), ConditionOperator::Minor)
                } else if args[*counter + 2].eq("<=") {
                    build_condition(x[0], args[*counter + 2].to_string(), ConditionOperator::MinorEqual)
                } else if args[*counter + 2].eq("=") {
                    build_condition(x[0], args[*counter + 2].to_string(), ConditionOperator::Equal)
                } else if args[*counter + 2].eq(">=") {
                    build_condition(x[0], args[*counter + 2].to_string(), ConditionOperator::HigherEqual)
                } else if args[*counter + 2].eq(">") {
                    build_condition(x[0], args[*counter + 2].to_string(), ConditionOperator::Higher)
                } else {
                    return Err(WHERE_MAL_FORMATEADO)
                };
                new_node = build_simple_condition(Some(simple_condition));
                *counter += 3;
            }
        }
    } else {
        return Err(WHERE_MAL_FORMATEADO)
    }
    Ok(new_node)
}
*/
