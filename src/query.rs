pub mod query_type;

use query_type::QueryType;

use crate::condition::Condition;

// use crate::condition::ComplexCondition;

pub static DELETE_MIN_LEN: usize = 7; // DELETE FROM tabla WHERE a < b
pub static INSERT_MIN_LEN: usize = 6; // INSERT INTO tabla col VALUES val
pub static SELECT_MIN_LEN: usize = 4; // SELECT * FROM tabla
pub static UPDATE_MIN_LEN: usize = 10; // UPDATE tabla SET col = valor WHERE a < b

pub struct Query {
    pub operation: Option<QueryType>, // DELETE, INSERT, SELECT, UPDATE
    pub table: Option<String>,        // DELETE, INSERT, SELECT, UPDATE
    pub columns: Option<Vec<String>>, // INSERT, SELECT, UPDATE
    pub where_condition: Option<Condition>, // DELETE (always), SELECT (sometimes), UPDATE (always) .
    pub order_by: Option<(String, bool)>,   // SELECT (sometimes)
    pub values: Option<Vec<String>>,        // INSERT, UPDATE (in update is set)
}

pub fn build_empty_query() -> Query {
    Query {
        operation: None,
        table: None,
        columns: None,
        where_condition: None,
        order_by: None,
        values: None,
    }
}
