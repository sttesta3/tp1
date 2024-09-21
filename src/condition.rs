pub mod condition_type;
/*
pub mod complex_condition;

use crate::libs::error;
use condition_type::BooleanOperator;
*/

use condition_type::ConditionOperator;

pub struct Condition {
    pub condition: ConditionOperator,
    pub column: Option<String>,
    pub value: Option<String>,
}

pub fn build_condition(column: String, value: String, cond: ConditionOperator) -> Condition {
    Condition {
        condition: cond,
        column: Some(column),
        value: Some(value),
    }
}

pub fn operate_condition(v1: &String, v2: &String, operator: &ConditionOperator) -> bool {
    if let Ok(x1) = v1.parse::<i32>() {
        if let Ok(x2) = v2.parse::<i32>() {
            match operator {
                ConditionOperator::Minor => return x1 < x2,
                ConditionOperator::MinorEqual => return x1 <= x2,
                ConditionOperator::Equal => return x1 == x2,
                ConditionOperator::Higher => return x1 > x2,
                ConditionOperator::HigherEqual => return x1 >= x2,
            }
        };
    };

    match operator {
        ConditionOperator::Minor => v1 < v2,
        ConditionOperator::MinorEqual => v1 <= v2,
        ConditionOperator::Equal => v1.eq(v2),
        ConditionOperator::Higher => v1 > v2,
        ConditionOperator::HigherEqual => v1 >= v2,
    }
}
