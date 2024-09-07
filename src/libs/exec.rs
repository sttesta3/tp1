fn exec_query(query: query::Query) -> Result<u32,u32> {
    match query.table {
        Some(f) => match std::fs::File::open(f) {
                Ok(x) => {
                    match query.operation.unwrap() {
                        QueryType::DELETE => exec_query_delete(x, query),
                        QueryType::INSERT => exec_query_insert(x, query),
                        QueryType::SELECT => exec_query_select(x, query),
                        QueryType::UPDATE => exec_query_update(x, query),
                    }
                }, 
                Err(_) => return Err(6)
            } ,
        None => return Err(6)
    }

}