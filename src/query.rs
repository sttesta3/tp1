pub mod query_type;

use query_type::QueryType;
pub struct Query {
    pub operation:          Option<QueryType>,
    pub table:              Option<String>,         // DELETE, INSERT, SELECT, UPDATE
    pub columns:            Option<Vec<String>>,    // INSERT, SELECT, UPDATE
    pub where_condition:    Option<Vec<String>>,         // DELETE (siempre), SELECT (a veces), UPDATE (siempre) 
    pub order_by:           Option<(String,bool)>,  // SELECT (a veces)
    pub values:             Option<Vec<String>>     // INSERT y UPDATE (en update es set)
}

pub fn build_empty_query() -> Query {
    return Query {
        operation:          None,
        table:              None,         
        columns:            None,    
        where_condition:    None,     
        order_by:           None,     
        values:             None,          
    }
}