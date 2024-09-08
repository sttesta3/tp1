pub mod query_type;

use query_type::QueryType;

use crate::condition::ComplexCondition;

pub struct Query {
    pub operation:          Option<QueryType>,          // DELETE, INSERT, SELECT, UPDATE
    pub table:              Option<String>,             // DELETE, INSERT, SELECT, UPDATE
    pub columns:            Option<Vec<String>>,        // INSERT, SELECT, UPDATE
    pub where_condition:    Option<ComplexCondition>,   // DELETE (always), SELECT (sometimes), UPDATE (always) . Tree of conditions
    pub order_by:           Option<(String,bool)>,      // SELECT (sometimes)
    pub values:             Option<Vec<String>>         // INSERT, UPDATE (in update is set)
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
