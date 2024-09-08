pub mod condition_type;
use crate::query::Query;
/* 
pub mod complex_condition;

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

/* 
pub fn operate_condition(filter: (&u32,&ConditionOperator,&String), elements: &Vec<String>) -> bool {
    let (column, operator, value) = filter;
    match operator {
        ConditionOperator::Minor => {},
        ConditionOperator::MinorEqual => {},
        ConditionOperator::Equal => {},
        ConditionOperator::Higher => {},
        ConditionOperator::HigherEqual => {},
    }
}
*/