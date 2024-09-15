use std::fs::{remove_file, rename, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use crate::condition::{operate_condition, Condition};
//use crate::condition::operate_condition;
use crate::query::query_type::QueryType;
use crate::query::Query;

use super::parsing::get_file_first_line;

pub fn exec_query(query: Query) {
    if let Some(op) = &query.operation {
        match op {
            QueryType::DELETE => exec_query_delete(query),
            QueryType::INSERT => exec_query_insert(query),
            QueryType::SELECT => exec_query_select(query),
            QueryType::UPDATE => exec_query_update(query),
        }
    }
}

fn exec_query_delete(query: Query) {
    if let Some(cond) = &query.where_condition {
        if let Some(value) = &cond.value {
            if let Some(table) = &query.table {
                // Check conditions for query. if not, don't do anything
                let tmp_file_name = get_tmp_file_name(table);
                let mut valid_operation = true;
                match File::open(table) {
                    Ok(file) => match File::create(&tmp_file_name) {
                        Ok(mut tmp_file) => {
                            let col_index = find_filter_column(&query);
                            let mut reader: BufReader<File> = BufReader::new(file);
                            let mut line = String::new();

                            let mut read = true;
                            while read {
                                if let Ok(x) = reader.read_line(&mut line) {
                                    line = line.replace('\n', "");
                                    read = x != 0;
                                    if read {
                                        let elements = line_to_vec(&line);
                                        if !operate_condition(
                                            &elements[col_index as usize],
                                            value,
                                            &cond.condition,
                                        ) {
                                            line.push('\n');
                                            if let Err(_error) = tmp_file.write(line.as_bytes()) {
                                                read = false;
                                                valid_operation = false;
                                            }
                                        }
                                    }
                                    line.clear();
                                } else {
                                    read = false;
                                    valid_operation = false;
                                }
                            }
                        }
                        Err(_) => valid_operation = false,
                    },
                    Err(_) => valid_operation = false,
                }

                if remove_old_file(table, &tmp_file_name, valid_operation).is_err() {
                    println!("Error en manipulación de archivos");
                }
            }
        }
    }
}

fn remove_old_file(file: &String, tmp_file: &String, valid_operation: bool) -> Result<u32, u32> {
    if valid_operation {
        if remove_file(file).is_ok() {
            if rename(tmp_file, file).is_err() {
                return Err(3);
            }
            Ok(0)
        } else {
            let _ = remove_file(tmp_file);
            Err(1)
        }
    } else {
        let _ = remove_file(tmp_file);
        Err(2)
    }
}

fn get_tmp_file_name(table: &str) -> String {
    // Pre: Path and name to file. ruta/a/tablas/tabla.csv
    // Post: same file but starting with period, as long as it's a hidden file
    let mut output = String::new();
    let last_item_index = table.split('/').enumerate().count() - 1;
    for (counter, element) in table.split('/').enumerate() {
        if counter == last_item_index {
            output.push('.');
        }

        output.push_str(element);

        if counter < last_item_index {
            output.push('/');
        }
    }
    output
}

fn exec_query_insert(query: Query) {
    if let Some(table) = &query.table {
        if let Some(write_columns) = &query.columns {
            let total_columns = find_total_columns(&query);
            //        let write_columns = find_print_columns(&query);

            match &mut OpenOptions::new().append(true).open(table) {
                Ok(file) => {
                    let mut write_line = String::new();
                    let mut counter = 0;
                    let mut writen_elements = 0;
                    while counter < total_columns {
                        if write_columns.contains(&counter) {
                            if let Some(x) = &query.values {
                                write_line.push_str(&x[writen_elements].to_string());
                                writen_elements += 1;
                            }
                        }

                        counter += 1;
                        if counter < total_columns {
                            write_line.push(',');
                        }
                    }

                    write_line.push('\n');
                    let _ = file.write(write_line.as_bytes());
                }
                Err(_) => println!("ERROR en el programa"),
            }
        }
    }
}

fn exec_query_select(query: Query) {
    match &query.order_by {
        Some(_) => exec_query_select_order_by(query),
        None => {
            let col_index = find_filter_column(&query);
            //            let print_columns: Vec<usize> = find_print_columns(&query);
            read_and_print_file(&query, col_index);
        }
    }
}

fn find_total_columns(query: &Query) -> usize {
    match get_file_first_line(query) {
        Some(line) => line_to_vec(&line).len(),
        None => 0,
    }
}

fn find_filter_column(query: &Query) -> i32 {
    let mut col_index_filter = -1;
    match &query.where_condition {
        Some(x) => match &x.column {
            Some(column) => match &query.table {
                Some(table) => match File::open(table) {
                    Ok(file) => {
                        let mut reader: BufReader<File> = BufReader::new(file);
                        let mut line = String::new();
                        match reader.read_line(&mut line) {
                            Ok(_) => {
                                line = line.replace('\n', "");
                                let mut split = line.split(',');
                                let mut element_opt = split.next();
                                let mut counter = 0;

                                while element_opt.is_some() && col_index_filter < 0 {
                                    if let Some(x) = element_opt {
                                        if x.eq(column) {
                                            col_index_filter = counter;
                                        }
                                    }
                                    counter += 1;
                                    element_opt = split.next();
                                }
                            }
                            Err(_) => col_index_filter = -1,
                        }
                    }
                    Err(_) => return -1,
                },
                None => return -1,
            },
            None => {
                col_index_filter = -1;
            }
        },
        None => col_index_filter = -1,
    }
    col_index_filter
}

fn read_and_print_file(query: &Query, col_filter: i32) {
    if let Some(table) = &query.table {
        if let Ok(f) = File::open(table) {
            let mut reader: BufReader<File> = BufReader::new(f);
            let mut line = String::new();

            let mut read = true;
            while read {
                if let Ok(x) = reader.read_line(&mut line) {
                    line = line.replace('\n', "");
                    read = x != 0;
                    if read {
                        let elements = line_to_vec(&line);
                        match &query.where_condition {
                            Some(condition) => print_file_conditioned(
                                &query.columns,
                                (col_filter, condition),
                                &elements,
                            ),
                            None => print_file_unconditional(&query.columns, &elements),
                        }
                        line.clear();
                    }
                }
            }
        }
    }
}

fn print_file_unconditional(columns_opt: &Option<Vec<usize>>, elements: &[String]) {
    // Pre: Columns vector sorted incremental && Elements of line content vector
    // Post: print to stdout the correct columns

    match columns_opt {
        Some(columns) => {
            // SELECT columns FROM
            let mut counter = 0;
            while counter < elements.len() {
                if columns.contains(&counter) {
                    if counter == 0 {
                        print!("{}", elements[counter]);
                    } else {
                        print!(",{}", elements[counter]);
                    }
                }

                counter += 1;
            }
        }
        None => {
            for (counter, element) in elements.iter().enumerate() {
                if counter == 0 {
                    print!("{}", element);
                } else {
                    print!(",{}", element);
                }
            }
        }
    }

    println!();
}

fn print_file_conditioned(
    columns_opt: &Option<Vec<usize>>,
    filter: (i32, &Condition),
    elements: &[String],
) {
    let (col_filter, condition) = filter;
    if let Some(value) = &condition.value {
        if operate_condition(&elements[col_filter as usize], value, &condition.condition) {
            print_file_unconditional(columns_opt, elements)
        }
    } else {
        print_file_unconditional(columns_opt, elements)
    }
}

fn line_to_vec(line: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut split = line.split(',');

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

fn exec_query_select_order_by(query: Query) {
    // TODO
}

fn exec_query_update(query: Query) {
    if let Some(cond) = &query.where_condition {
        if let Some(value) = &cond.value {
            if let Some(columns) = &query.columns {
                if let Some(values) = &query.values {
                    if let Some(table) = &query.table {
                        let tmp_file = get_tmp_file_name(table);
                        let mut valid_operation = true;
                        match File::open(table) {
                            Ok(file) => match File::create(&tmp_file) {
                                Ok(mut tmp_file) => {
                                    let col_index = find_filter_column(&query);
                                    let mut reader: BufReader<File> = BufReader::new(file);
                                    let mut line = String::new();

                                    let mut read = true;
                                    while read {
                                        if let Ok(x) = reader.read_line(&mut line) {
                                            read = x != 0;
                                            if read {
                                                line = line.replace('\n', "");
                                                let elements = line_to_vec(&line);
                                                if !operate_condition(
                                                    &elements[col_index as usize],
                                                    value,
                                                    &cond.condition,
                                                ) {
                                                    // Line not updated
                                                    line.push('\n');
                                                    if let Err(_error) =
                                                        tmp_file.write(line.as_bytes())
                                                    {
                                                        read = false;
                                                        valid_operation = false;
                                                    }
                                                } else {
                                                    // Update Line
                                                    if let Err(_error) = tmp_file.write(
                                                        update_line(&elements, columns, values)
                                                            .as_bytes(),
                                                    ) {
                                                        read = false;
                                                        valid_operation = false;
                                                    }
                                                }
                                            }
                                            line.clear();
                                        } else {
                                            read = false;
                                            valid_operation = false;
                                        }
                                    }
                                }
                                Err(_) => valid_operation = false,
                            },
                            Err(_) => valid_operation = false,
                        }

                        if remove_old_file(table, &tmp_file, valid_operation).is_err() {
                            println!("Error en manipulación de archivos");
                        }
                    }
                }
            }
        }
    }
}

fn update_line(elements: &[String], columns: &[usize], values: &[String]) -> String {
    let mut result = String::new();
    let mut writen_counter = 0;
    for (counter, element) in elements.iter().enumerate() {
        if counter > 0 {
            result.push(',');
        }
        if columns.contains(&counter) {
            result.push_str(&values[writen_counter].to_string());
            writen_counter += 1;
        } else {
            result.push_str(&element.to_string());
        }
    }
    result
}
