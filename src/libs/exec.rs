use std::fs::{remove_file, rename, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use crate::condition::{self, operate_condition, operate_full_condition, Condition};
//use crate::condition::operate_condition;
use crate::query::query_type::QueryType;
use crate::query::Query;

use super::error;
use super::parsing::{self, get_file_first_line, text_to_vec};

static FILE_SORT_BUFFER: usize = 3;

/// Main execution function, execs query
/// By definition, if something is wrong with the query, then it should do nothing
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

/// Exec query: Special case for delete
fn exec_query_delete(query: Query) {
    if let Some(cond) = &query.where_condition {
        if let Some(table) = &query.table {
            // Check conditions for query. if not, don't do anything
            let tmp_file_name = get_tmp_file_name(table);
            let mut valid_operation = true;
            match File::open(table) {
                Ok(file) => match File::create(&tmp_file_name) {
                    Ok(mut tmp_file) => {
                        let mut reader: BufReader<File> = BufReader::new(file);
                        let mut line = String::new();

                        let mut read = true;
                        while read {
                            if let Ok(x) = reader.read_line(&mut line) {
                                line = line.replace('\n', "");
                                read = x != 0;
                                if read {
                                    let elements = line_to_vec(&line);
                                    if !operate_full_condition(&elements, cond) {
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
                println!("ERROR: limpieza de archivos temporales");
            }
        }
    }
}

/// Exec query: Special case for insert
fn exec_query_insert(query: Query) {
    if let Some(table) = &query.table {
        if let Some(write_columns) = &query.columns {
            let total_columns = find_total_columns(&query);

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
                Err(_) => println!("Error: Manipulación de archivo temporal en INSERT"),
            }
        }
    }
}

/// Exec query: Special case for select
fn exec_query_select(query: Query) {
    print_header(&query);
    match &query.order_by {
        Some((column, asc)) => {
            if let Some(col_index) = find_column(&query, column) {
                if let Ok(files) = read_into_sorted_files(&query, col_index, asc) {
                    print_sorted_files(&files, &col_index, asc);
                    file_cleanup(&files);
                }
            }
        }
        None => {
            read_and_print_file(&query);
        }
    }
}

/// Cleans tmp files
/// Pre: filenames
/// Post: Cleans files
fn file_cleanup(files: &Vec<String>) {
    for file in files {
        if remove_file(file).is_err() {
            println!("ERROR: Limpieza de archivos temporales");
        }
    }
}

/// Pre: Name of tmp_files, column for ordering and selector of asc/desc
/// Post: Prints from tmp_files in order
fn print_sorted_files(files: &[String], col_index: &usize, asc: &bool) {
    if let Some(mut readers) = create_readers(files) {
        if let Some(mut lines_buffer) = readers_read_first_line(&mut readers) {
            // While there are lines to be printed
            while let Some(elements) =
                get_next_line(&mut lines_buffer, &mut readers, *col_index, asc)
            {
                let mut counter = 0;
                for element in elements {
                    if !element.eq("") {
                        if counter == 0 {
                            print!("{}", element);
                        } else {
                            print!(",{}", element);
                        }
                        counter += 1;
                    }
                }
                println!();
            }
        }
    }
}

/// Gets next sorted line from mem's buffer and replace it with the next line on that file
/// Pre: Lines buffer, readers, col index for sorting and ascending/descending bool
/// Post: Next line, if not fully read
fn get_next_line(
    lines_buffer: &mut Vec<Vec<String>>,
    readers: &mut [BufReader<File>],
    col_index: usize,
    asc: &bool,
) -> Option<Vec<String>> {
    let mut line = String::new();
    match find_next_line(lines_buffer, col_index, asc) {
        Some(reader_index) => match &readers[reader_index].read_line(&mut line) {
            Ok(x) => {
                if *x > 0 {
                    let _ = line.replace('\n', "");
                    lines_buffer.push(text_to_vec(&line, true));
                } else {
                    let empty_line_buf: Vec<String> = Vec::new();
                    lines_buffer.push(empty_line_buf);
                }
                Some(lines_buffer.swap_remove(reader_index))
            }
            Err(_) => {
                let empty_line_buf: Vec<String> = Vec::new();
                lines_buffer.push(empty_line_buf);
                Some(lines_buffer.swap_remove(reader_index))
            }
        },
        None => None,
    }
}

/// Find next sorted line's index to be printed
/// Pre: Lines buffer, col index and ascending/descending
/// Post: Index of next line, if any
fn find_next_line(lines_buffer: &mut [Vec<String>], col_index: usize, asc: &bool) -> Option<usize> {
    // Find first line
    let mut candidate = 0;
    while candidate < lines_buffer.len() && lines_buffer[candidate].is_empty() {
        candidate += 1;
    }

    if candidate == lines_buffer.len() {
        None
    } else {
        // Find lower line
        let mut counter = candidate + 1;
        while counter < lines_buffer.len() {
            if !lines_buffer[counter].is_empty() {
                if *asc {
                    if operate_condition(
                        &lines_buffer[counter][col_index],
                        &lines_buffer[candidate][col_index],
                        &condition::condition_type::ConditionOperator::Minor,
                    ) {
                        candidate = counter;
                    }
                } else if operate_condition(
                    &lines_buffer[counter][col_index],
                    &lines_buffer[candidate][col_index],
                    &condition::condition_type::ConditionOperator::Higher,
                ) {
                    candidate = counter;
                }
            }
            counter += 1;
        }

        Some(candidate)
    }
}

/// Read first line from all files
/// Pre: Lines buffer, col index and ascending/descending
/// Post: Index of next line, if any
fn readers_read_first_line(readers: &mut Vec<BufReader<File>>) -> Option<Vec<Vec<String>>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut line = String::new();

    for reader in readers {
        match reader.read_line(&mut line) {
            Ok(_) => {
                let _ = line.replace('\n', "");
                result.push(text_to_vec(&line, true));
                line.clear();
            }
            Err(_) => return None,
        }
    }
    Some(result)
}

/// Create readers for reading files  
/// Pre: Filenames  
/// Post: File's readers
fn create_readers(files: &[String]) -> Option<Vec<BufReader<File>>> {
    let mut valid = true;
    let mut readers: Vec<BufReader<File>> = Vec::new();

    let mut counter = 0;
    while counter < files.len() && valid {
        match File::open(&files[counter]) {
            Err(_) => valid = false,
            Ok(file) => {
                readers.push(BufReader::new(file));
                counter += 1;
            }
        }
    }

    if valid {
        Some(readers)
    } else {
        None
    }
}

/// Exec query: Special case for update
fn exec_query_update(query: Query) {
    if let Some(cond) = &query.where_condition {
        if let Some(columns) = &query.columns {
            if let Some(values) = &query.values {
                if let Some(table) = &query.table {
                    let tmp_file = get_tmp_file_name(table);
                    let mut valid_operation = true;
                    match File::open(table) {
                        Ok(file) => match File::create(&tmp_file) {
                            Ok(mut tmp_file) => {
                                let mut reader: BufReader<File> = BufReader::new(file);
                                let mut line = String::new();

                                let mut read = true;
                                while read {
                                    if let Ok(x) = reader.read_line(&mut line) {
                                        read = x != 0;
                                        if read {
                                            line = line.replace('\n', "");
                                            let elements = line_to_vec(&line);
                                            if !operate_full_condition(&elements, cond) {
                                                // Line not updated
                                                line.push('\n');
                                                if let Err(_error) = tmp_file.write(line.as_bytes())
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
                        println!("ERROR: Manipulación de archivo temporal en UPDATE");
                    }
                }
            }
        }
    }
}

/// Exec query: Special case for select
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

/// Aux Function. Remove old tmp file
fn remove_old_file(file: &String, tmp_file: &String, valid_operation: bool) -> Result<u32, u32> {
    if valid_operation {
        if remove_file(file).is_ok() {
            if rename(tmp_file, file).is_err() {
                return Err(error::ERROR_RENOMBRADO_ARCHIVO_TEMPORAL);
            }
            Ok(0)
        } else {
            let _ = remove_file(tmp_file);
            Err(error::ERROR_INTERCAMBIANDO_TEMPORAL_POR_OFICIAL)
        }
    } else {
        let _ = remove_file(tmp_file);
        Err(error::ERROR_OPERACION_INVALIDA_EN_EJECUCCION)
    }
}

/// Returns tmp filename from official file
/// Pre: Path and name to file. ruta/a/tablas/tabla.csv
/// Post: same file but starting with period, as long as it's a hidden file
fn get_tmp_file_name(table: &str) -> String {
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

/// Aux function. Total number of columns in table
fn find_total_columns(query: &Query) -> usize {
    match get_file_first_line(query) {
        Some(line) => line_to_vec(&line).len(),
        None => 0,
    }
}

/// Find column index given it's name
///
/// Pre: Valid query and column for text
/// Post: Index if any
fn find_column(query: &Query, column: &String) -> Option<usize> {
    match get_file_first_line(query) {
        Some(line) => {
            let args = parsing::text_to_vec(&line, true);
            let mut counter = 0;
            while counter < args.len() && !args[counter].eq(column) {
                counter += 1
            }

            if counter == args.len() {
                None
            } else {
                match &query.columns {
                    None => Some(counter),
                    Some(cols) => {
                        if cols.contains(&counter) {
                            Some(counter)
                        } else {
                            None
                        }
                    }
                }
            }
        }
        None => None,
    }
}

/// Print header of function.
fn print_header(query: &Query) {
    if let Some(table) = &query.table {
        if let Ok(file) = File::open(table) {
            let mut reader: BufReader<File> = BufReader::new(file);
            let mut line = String::new();

            if let Ok(x) = reader.read_line(&mut line) {
                if x > 0 {
                    match &query.columns {
                        Some(columns) => {
                            line = line.replace('\n', "");
                            let args = text_to_vec(&line, true);
                            let mut first = true;
                            for (counter, arg) in args.into_iter().enumerate() {
                                if columns.contains(&counter) {
                                    if first {
                                        print!("{}", arg);
                                        first = false;
                                    } else {
                                        print!(",{}", arg);
                                    }
                                }
                            }
                            println!();
                        }
                        None => print!("{}", line),
                    }
                }
            }
        }
    }
}

/// Function for printing file ( special select case when no sort by )
fn read_and_print_file(query: &Query) {
    if let Some(table) = &query.table {
        if let Ok(f) = File::open(table) {
            let mut reader: BufReader<File> = BufReader::new(f);
            let mut line = String::new();

            let mut read = reader.read_line(&mut line).is_ok();
            line.clear();
            while read {
                if let Ok(x) = reader.read_line(&mut line) {
                    line = line.replace('\n', "");
                    read = x != 0;
                    if read {
                        let elements = line_to_vec(&line);
                        match &query.where_condition {
                            Some(condition) => {
                                print_file_conditioned(&query.columns, condition, &elements)
                            }
                            None => print_file_unconditional(&query.columns, &elements),
                        }
                        line.clear();
                    }
                }
            }
        }
    }
}

/// Exec query line by line, sorting the output in buffer memory and printing to tmp files
/// PD: Similar to Cassandra Write Path :D
///
/// Pre:  Query, the col index for sorting and bool of ascending/descending
/// Post: Vec of tmp_files
fn read_into_sorted_files(query: &Query, col_index: usize, asc: &bool) -> Result<Vec<String>, u32> {
    match &query.table {
        None => Err(3),
        Some(table) => {
            let tmp_file_name = get_tmp_file_name(table);
            match File::open(table) {
                Err(_) => Err(3),
                Ok(table_file) => {
                    let mut tmp_filenames: Vec<String> = Vec::new();
                    let mut lines_buffer: Vec<Vec<String>> = Vec::new(); // list maybe better ?

                    let mut reader: BufReader<File> = BufReader::new(table_file);
                    let mut line = String::new();

                    let mut read: bool = true;
                    let mut valid_operation: bool = true;
                    if reader.read_line(&mut line).is_ok() {
                        line.clear();
                        while read {
                            match reader.read_line(&mut line) {
                                Err(_) => {
                                    read = false;
                                    valid_operation = false;
                                }
                                Ok(x) => {
                                    line = line.replace('\n', "");
                                    read = x != 0;
                                    if read {
                                        let elements = text_to_vec(&line, true);
                                        match &query.where_condition {
                                            Some(cond) => insert_conditioned(
                                                elements,
                                                cond,
                                                &query.columns,
                                                &mut lines_buffer,
                                                &col_index,
                                                asc,
                                            ),
                                            None => insert_unconditioned(
                                                elements,
                                                &query.columns,
                                                &mut lines_buffer,
                                                &col_index,
                                                asc,
                                            ),
                                        }
                                        line.clear();
                                    }

                                    if lines_buffer.len() == FILE_SORT_BUFFER {
                                        write_to_tmp_file(
                                            &tmp_file_name,
                                            &mut tmp_filenames,
                                            &lines_buffer,
                                            &mut read,
                                            &mut valid_operation,
                                        );
                                        lines_buffer.clear();
                                    }
                                }
                            }
                        }

                        if valid_operation {
                            if !lines_buffer.is_empty() {
                                write_to_tmp_file(
                                    &tmp_file_name,
                                    &mut tmp_filenames,
                                    &lines_buffer,
                                    &mut read,
                                    &mut valid_operation,
                                );
                            }
                            Ok(tmp_filenames)
                        } else {
                            Err(3)
                        }
                    } else {
                        Err(error::ARCHIVO_VACIO)
                    }
                }
            }
        }
    }
}

/// Write the buffer to file
fn write_to_tmp_file(
    tmp_file_name: &String,
    tmp_filenames: &mut Vec<String>,
    lines_buffer: &Vec<Vec<String>>,
    read: &mut bool,
    valid_operation: &mut bool,
) {
    let mut new_tmp_file_name = String::from(tmp_file_name);
    new_tmp_file_name.push_str(format!(".{}", &tmp_filenames.len()).as_str());

    match File::create(&new_tmp_file_name) {
        Ok(mut tmp_f) => {
            for elements in lines_buffer {
                let mut first = true;
                for element in elements {
                    if first {
                        let _ = tmp_f.write(element.as_bytes());
                        first = false;
                    } else {
                        let _ = tmp_f.write(format!(",{}", element).as_bytes());
                    }
                }
                let _ = tmp_f.write("\n".as_bytes());
            }
            tmp_filenames.push(new_tmp_file_name);
        }
        Err(_) => {
            *read = false;
            *valid_operation = false;
        }
    }
}

/// Insert line to buffer in sorted position (special case: no boolean condition)
fn insert_unconditioned(
    elements: Vec<String>,
    columns_opt: &Option<Vec<usize>>,
    lines_buffer: &mut Vec<Vec<String>>,
    col_index: &usize,
    asc: &bool,
) {
    let position: usize = if lines_buffer.is_empty() {
        0
    } else {
        find_insert_position(
            &elements,
            lines_buffer,
            0,
            lines_buffer.len() - 1,
            *col_index,
            asc,
        )
    };

    match columns_opt {
        Some(columns) => {
            // SELECT columns FROM
            let mut vector: Vec<String> = Vec::new();

            let mut counter = 0;
            while counter < elements.len() {
                if columns.contains(&counter) {
                    vector.push(elements[counter].to_string());
                }

                counter += 1;
            }
            lines_buffer.insert(position, vector);
        }
        None => lines_buffer.insert(position, elements),
    }
}

/// Insert line to buffer in sorted position (special case: there's a boolean condition)
fn insert_conditioned(
    elements: Vec<String>,
    condition: &[Vec<Condition>],
    columns_opt: &Option<Vec<usize>>,
    lines_buffer: &mut Vec<Vec<String>>,
    col_index: &usize,
    asc: &bool,
) {
    if operate_full_condition(&elements, condition) {
        insert_unconditioned(elements, columns_opt, lines_buffer, col_index, asc);
    }
}

/// Binary search of elements position.
fn find_insert_position(
    elements: &Vec<String>,
    lines_buffer: &Vec<Vec<String>>,
    min_pos: usize,
    max_pos: usize,
    col_index: usize,
    asc: &bool,
) -> usize {
    let less_than_minor = operate_condition(
        &elements[col_index],
        &lines_buffer[min_pos][col_index],
        &condition::condition_type::ConditionOperator::Minor,
    );
    let less_than_med = operate_condition(
        &elements[col_index],
        &lines_buffer[(min_pos + max_pos) / 2][col_index],
        &condition::condition_type::ConditionOperator::Minor,
    );

    if min_pos == max_pos {
        if less_than_minor {
            if *asc {
                min_pos
            } else {
                min_pos + 1
            }
        } else if *asc {
            min_pos + 1
        } else {
            min_pos
        }
    } else if min_pos + 1 == max_pos {
        if less_than_minor {
            if *asc {
                find_insert_position(elements, lines_buffer, min_pos, min_pos, col_index, asc)
            } else {
                find_insert_position(elements, lines_buffer, max_pos, max_pos, col_index, asc)
            }
        } else if *asc {
            find_insert_position(elements, lines_buffer, max_pos, max_pos, col_index, asc)
        } else {
            find_insert_position(elements, lines_buffer, min_pos, min_pos, col_index, asc)
        }
    } else {
        let med = (min_pos + max_pos) / 2;
        if *asc {
            if less_than_med {
                find_insert_position(elements, lines_buffer, min_pos, med, col_index, asc)
            } else {
                find_insert_position(elements, lines_buffer, med, max_pos, col_index, asc)
            }
        } else if less_than_med {
            find_insert_position(elements, lines_buffer, med, max_pos, col_index, asc)
        } else {
            find_insert_position(elements, lines_buffer, min_pos, med, col_index, asc)
        }
    }
}

/// Print line to stdout ( special case, no SORT BY condition )
/// Pre: Columns vector sorted && Elements of line content vector
/// Post: print to stdout the correct columns
fn print_file_unconditional(columns_opt: &Option<Vec<usize>>, elements: &[String]) {
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

/// Print line to stdout with a boolean condition( special case, no SORT BY condition )
fn print_file_conditioned(
    columns_opt: &Option<Vec<usize>>,
    condition: &[Vec<Condition>],
    elements: &[String],
) {
    if operate_full_condition(elements, condition) {
        print_file_unconditional(columns_opt, elements)
    }
}

/// Aux function. Tokenizer for special cases
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsearch1() {
        let elements = vec![
            String::from("4"),
            String::from("jorge"),
            String::from("martell"),
        ];
        let lines_buffer = vec![
            vec![
                String::from("1"),
                String::from("diego"),
                String::from("mayor"),
            ],
            vec![
                String::from("3"),
                String::from("pamela"),
                String::from("ureta"),
            ],
            vec![
                String::from("5"),
                String::from("lucas"),
                String::from("catini"),
            ],
        ];
        let rt1 = find_insert_position(
            &elements,
            &lines_buffer,
            0,
            lines_buffer.len() - 1,
            0,
            &true,
        );
        assert_eq!(rt1, 2);
    }

    #[test]
    fn test_bsearch2() {
        let elements = vec![
            String::from("1"),
            String::from("Jorge"),
            String::from("martell"),
        ];
        let lines_buffer = vec![
            vec![
                String::from("4"),
                String::from("María"),
                String::from("Rodríguez"),
            ],
            vec![
                String::from("10"),
                String::from("Manuel"),
                String::from("Allen"),
            ],
            vec![
                String::from("6"),
                String::from("Laura"),
                String::from("Fernández"),
            ],
            vec![
                String::from("5"),
                String::from("José"),
                String::from("López"),
            ],
            vec![
                String::from("7"),
                String::from("Diego"),
                String::from("Torres"),
            ],
            vec![
                String::from("3"),
                String::from("Carlos"),
                String::from("Gómez"),
            ],
            vec![
                String::from("2"),
                String::from("Ana"),
                String::from("López"),
            ],
        ];
        let rt2 = find_insert_position(
            &elements,
            &lines_buffer,
            0,
            lines_buffer.len() - 1,
            1,
            &false,
        );
        assert_eq!(rt2, 4);
    }
}

/*
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

fn save_file_unconditional(columns_opt: &Option<Vec<usize>>, elements: &[String], file: &mut File) {
    // Pre: Columns vector sorted incremental && Elements of line content vector
    // Post: print to stdout the correct columns

    match columns_opt {
        Some(columns) => {
            // SELECT columns FROM
            let mut counter = 0;
            while counter < elements.len() {
                if columns.contains(&counter) {
                    if counter == 0 {
                        file.write(elements[counter].as_bytes());
                    } else {
                        file.write(format!(",{}", elements[counter]).as_bytes());
                    }
                }

                counter += 1;
            }
        }
        None => {
            for (counter, element) in elements.iter().enumerate() {
                if counter == 0 {
                    file.write(element.as_bytes());
                } else {
                    file.write(format!(",{}", element).as_bytes());
                }
            }
        }
    }

    println!();
}

fn save_file_conditioned(
    columns_opt: &Option<Vec<usize>>,
    filter: (i32, &Condition),
    elements: &[String],
    file: &mut File,
) {
    let (col_filter, condition) = filter;
    if let Some(value) = &condition.value {
        if operate_condition(&elements[col_filter as usize], value, &condition.condition) {
            save_file_unconditional(columns_opt, elements, file)
        }
    } else {
        save_file_unconditional(columns_opt, elements, file)
    }
}

fn read_and_save_file(query: &Query, col_filter: i32) -> Result<usize, u32> {
    // Pre:  query, filter and
    // Post:
    if let Some(table) = &query.table {
        let tmp_file_name = get_tmp_file_name(table);
        if let Ok(f) = File::open(table) {
            match File::create(tmp_file_name) {
                Ok(mut tmp_f) => {
                    let mut line = String::new();
                    let mut reader: BufReader<File> = BufReader::new(f);

                    let mut read = true;
                    while read {
                        if let Ok(x) = reader.read_line(&mut line) {
                            line = line.replace('\n', "");
                            read = x != 0;
                            if read {
                                let elements = line_to_vec(&line);
                                match &query.where_condition {
                                    Some(condition) => save_file_conditioned(
                                        &query.columns,
                                        (col_filter, condition),
                                        &elements,
                                        &mut tmp_f,
                                    ),
                                    None => save_file_unconditional(
                                        &query.columns,
                                        &elements,
                                        &mut tmp_f,
                                    ),
                                }
                                line.clear();
                            }
                        }
                    }
                    return Ok(0);
                }
                Err(_) => Err(1),
            }
        } else {
            return Err(1);
        }
    } else {
        return Err(1);
    }
}

fn sort_and_print_file(
    column_number: usize,
    query: &Query,
    asc: &bool,
) -> Result<Vec<String>, u32> {
    // Pre: Column number for sorting
    // Post: Separates into files, sort them and print them in correct order. Result name of files for cleanup
    match &query.table {
        None => Err(3),
        Some(table) => {
            let tmp_unsorted_filename = get_tmp_file_name(table);
            match File::open(tmp_unsorted_filename) {
                Err(_) => Err(3),
                Ok(unsorted_file) => {
                    let mut reader: BufReader<File> = BufReader::new(unsorted_file);
                    let mut line = String::new();
                    let mut actually_sorting_elements: Vec<Vec<String>> = Vec::new();

                    let mut sorted_files: Vec<File> = Vec::new();
                    let mut sorted_filenames: Vec<String> = Vec::new();
                    let mut counter = 0;

                    let mut read = true;
                    let mut valid_operation = true;
                    while read {
                        if let Ok(x) = reader.read_line(&mut line) {
                            line = line.replace('\n', "");
                            read = x != 0;

                            let element = text_to_vec(&line, true);
                            if counter < FILE_SORT_BUFFER {
                                actually_sorting_elements.insert(
                                    find_sorted_position(
                                        &actually_sorting_elements,
                                        &element,
                                        column_number,
                                        asc,
                                    ),
                                    element,
                                );
                            } else {
                                let mut new_file = String::from(format!(".{}", sorted_files.len()));
                                new_file.push_str(&tmp_unsorted_filename);
                                match File::create(new_file) {
                                    Ok(f) => {
                                        for element in actually_sorting_elements {
                                            let mut write_line = String::new();
                                            for (counter, word) in element.iter().enumerate() {
                                                if counter == 0 {
                                                    f.write(word.as_bytes());
                                                } else {
                                                    f.write(format!(",{}", &word));
                                                }
                                            }
                                        }
                                        sorted_files.push(f);
                                        sorted_filenames.push(new_file);
                                        actually_sorting_elements.clear();
                                    }
                                    Err(_) => {
                                        read = false;
                                        valid_operation = false;
                                    }
                                }
                            }
                        } else {
                            valid_operation = false;
                            read = false;
                        }
                    }

                    if valid_operation {
                        Ok(sorted_filenames)
                    } else {
                        Err(3)
                    }
                }
            }
        }
    }
}


fn file_cleanup(files: Vec<String>) {
    for file in files {
        let _ = remove_file(file);
    }
}

fn find_sorted_position(
    sorted_vec: &Vec<Vec<String>>,
    new_element: &Vec<String>,
    column_number: usize,
    asc: &bool,
) -> usize {
    // Insert into shifts everything to right.
    0
}

fn insert_conditioned(elements: &[String], columns_opt: &Option<Vec<usize>>, lines_buffer: &mut Vec<Vec<String>>, col_index: &usize, filter: &Condition) {
let (col_filter, condition) = filter;
    if let Some(value) = &condition.value {
        if operate_condition(&elements[col_filter as usize], value, &condition.condition) {
            print_file_unconditional(columns_opt, elements)
        }
    } else {
        print_file_unconditional(columns_opt, elements)
    }
}

fn exec_query_select(query: Query) {
    match &query.order_by {
        Some((column, asc)) => {
            let col_index: i32 = find_filter_column(&query);
            if read_and_save_file(&query, col_index).is_ok() {
                if let Some(column_number) = find_column(&query, column) {
                    if let Ok(tmp_files) = sort_and_print_file(column_number, &query, asc) {
                        file_cleanup(tmp_files);
                    }
                }
            }
        }
        None => {
            let col_index: i32 = find_filter_column(&query);
            read_and_print_file(&query, col_index);
        }
    }
}

fn get_next_line(lines_buffer: &mut Vec<Vec<String>>, readers: &mut Vec<BufReader<File>>, col_index: usize, asc: &bool) -> Option<Vec<String>> {
    // Post: Next line in order, if not fully read
    let mut line = String::new();
    match find_available_reader(&readers) {
        None => None,
        Some(mut candidate) => {
            let mut counter = candidate + 1;
            while counter < readers.len() {
                if readers[counter].0 < FILE_SORT_BUFFER && counter != candidate {
                    if *asc {
                        if lines_buffer[counter][col_index] < lines_buffer[candidate][col_index] {
                            candidate = counter;
                        }
                    } else {
                        if lines_buffer[counter][col_index] > lines_buffer[candidate][col_index] {
                            candidate = counter;
                        }
                    }
                }

                counter += 1;
            }

            if counter == readers.len() {
                None
            } else {
                let new_line: Vec<String>;
                match readers[candidate].1.read_line(&mut line){
                    Err(_) => new_line = Vec::new(),
                    Ok(read_result) => {
                        if read_result == 0 {
                            new_line = Vec::new();  // If this happen, then never read again from here
                            readers[candidate].0 = FILE_SORT_BUFFER;
                        } else {
                            lines_buffer.push(text_to_vec(&line, true));
                            let len = lines_buffer.len();
                            lines_buffer.swap(candidate,len - 1);
                            match lines_buffer.pop() {
                                Some(x) => {
                                    new_line = x;
                                    readers[candidate].0 += 1;
                                },
                                None => {
                                    new_line = Vec::new();  // If this happen, then never read again from here
                                    readers[candidate].0 = FILE_SORT_BUFFER;
                                }
                            }
                        }
                    }
                }

                Some(new_line)
            }
        }
    }
}

fn find_available_reader(readers: &Vec<BufReader<File>>) -> Option<usize> {
    // Pre: Readers
    // Post: First available reader position
    let mut counter = 0;
    while counter < readers.len() && ! &readers[counter].0 < FILE_SORT_BUFFER {
        counter += 1;
    }

    if counter == readers.len() {
        None
    } else {
        Some(counter)
    }
}
*/
