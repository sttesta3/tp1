pub mod condition_type;
/* 
pub mod complex_condition;

use crate::query::Query;
use crate::libs::error;
use condition_type::BooleanOperator;
*/

use condition_type::ConditionOperator;

pub struct Condition {
    pub condition:  ConditionOperator,
    pub column:     Option<String>,
    pub value:      Option<String>,
}

pub fn build_condition(column: String, value: String, condition: ConditionOperator) -> Condition {
    return Condition {
        condition:  condition,
        column:     Some(column),
        value:      Some(value),
    }
}
