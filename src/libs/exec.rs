use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use crate::condition::{self, operate_condition, Condition};
//use crate::condition::operate_condition;
use crate::query::query_type::QueryType;
use crate::query::Query;

use super::parsing::get_file_first_line;

pub fn exec_query(query: Query) {
    match &query.operation {
        Some(op) => match op {
            QueryType::DELETE => exec_query_delete(query),
            QueryType::INSERT => exec_query_insert(query),
            QueryType::SELECT => exec_query_select(query),
            QueryType::UPDATE => exec_query_update(query),
        },
        None => println!("Error en el programa"),
    }
}

fn exec_query_delete(query: Query) {
    // TODO
}

fn exec_query_insert(query: Query) {
    match &query.table {
        Some(table) => {
            let total_columns = find_total_columns(&query);
            let write_columns = find_print_columns(&query);
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
        None => println!("ERROR en el programa"),
    }
}

fn exec_query_select(query: Query) {
    match &query.order_by {
        Some(_) => exec_query_select_order_by(query),
        None => {
            let col_index = find_filter_column(&query);
            let print_columns: Vec<usize> = find_print_columns(&query);
            read_and_print_file(&query, col_index, &print_columns);
        }
    }
}

fn find_total_columns(query: &Query) -> usize {
    match get_file_first_line(query) {
        Some(line) => line_to_vec(&line).len(),
        None => 0,
    }
}

fn find_print_columns(query: &Query) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    match &query.columns {
        Some(cols) => match get_file_first_line(query) {
            Some(line) => {
                let table_columns: Vec<String> = line_to_vec(&line);
                for element in cols {
                    let mut counter = 0;
                    while !table_columns[counter].eq(element) && counter < table_columns.len() {
                        counter += 1;
                    }
                    if counter < table_columns.len() {
                        result.push(counter);
                    }
                }

                result
            }
            None => result,
        },
        None => result,
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

fn read_and_print_file(query: &Query, col_filter: i32, columns: &[usize]) {
    match &query.table {
        Some(table) => match File::open(table) {
            Ok(f) => {
                let mut reader: BufReader<File> = BufReader::new(f);
                let mut line = String::new();

                // Print header
                /*                 match reader.read_line(&mut line) {
                                    Ok(_) => {
                                        line = line.replace('\n', "");
                                        println!("{}",line);
                                        line.clear();
                                    },
                                    Err(_) => println!("Error en el programa")  // TODO
                                }
                */
                // Print other lines
                let mut read = true;
                while read {
                    match reader.read_line(&mut line) {
                        Ok(x) => {
                            line = line.replace('\n', "");
                            read = x != 0;
                            if read {
                                let elements = line_to_vec(&line);
                                match &query.where_condition {
                                    Some(condition) => print_file_conditioned(columns, (col_filter, condition), &elements),
                                    None => print_file_unconditional(columns, &elements),
                                }
                                line.clear();
                            }
                        }
                        Err(_) => println!("Error en el programa"),
                    }
                }
            }
            Err(_) => println!("Error en el programa"),
        },
        None => println!("Error en el programa "),
    }
}

fn print_file_unconditional(columns: &[usize], elements: &[String]) {
    // Pre: Columns vector sorted incremental && Elements of line content vector
    // Post: print to stdout the correct columns

    let mut first = true;
    if columns.is_empty() { // SELECT * FROM
        for (counter, element) in elements.iter().enumerate() {
            if first {
                print!("{}", element);
                first = false;
            } else {
                print!(",{}", element);
            }
        }
    } else {                // SELECT columns FROM
        let mut counter = 0;
        while counter < elements.len() {
            if columns.contains(&counter) {
                if first {
                    print!("{}", elements[counter]);
                    first = false;
                } else {
                    print!(",{}", elements[counter]);
                }
            }

            counter += 1;
        }
    }
    println!();
}

fn print_file_conditioned(columns: &[usize], filter: (i32, &Condition), elements: &[String]) {
    let (col_filter,condition) = filter;
    if let Some(value) = &condition.value {
        if operate_condition(value, &elements[col_filter as usize], &condition.condition) {
            print_file_unconditional(columns, elements)
        }
    } else {
        print_file_unconditional(columns, elements)
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

fn exec_query_select_order_by(query: Query) {}

fn exec_query_update(query: Query) {
    // TODO
}
