pub mod condition_type;
// pub mod complex_condition;
// use condition_type::BooleanOperator;
// use crate::libs::error;

use condition_type::ConditionOperator;

pub struct Condition {
    pub condition: ConditionOperator,
    pub column: Option<usize>,
    pub value: Option<String>,
}

/// Constructor for not condition 
pub fn build_not_condition() -> Condition {
    Condition {
        condition: ConditionOperator::Equal,
        column: None,
        value: None,
    }
}

/// Constructor for general condition (simple expression, non boolean) 
pub fn build_condition(column: usize, value: String, cond: ConditionOperator) -> Condition {
    Condition {
        condition: cond,
        column: Some(column),
        value: Some(value),
    }
}

/// Returns string (or int if it's parseable as int) comparison
/// 
/// Pre:  Strings and operator
/// Post: True if condition applies 
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

/// Returns the result of applying the condition to the lineW
/// 
/// Pre:  Line to vec and condition (bool vector).
/// Post: Bool if condition applies or not
pub fn operate_full_condition(elements: &[String], condition: &[Vec<Condition>]) -> bool {

    let mut or_valid = false;
    let mut or_counter = 0;

    let mut and_valid;
    let mut and_counter;
    let mut not_detected;

    while or_counter < condition.len() && !or_valid {
        and_valid = true;
        and_counter = 0;
        not_detected = false;
        while and_counter < condition[or_counter].len() && and_valid {
            match &condition[or_counter][and_counter].column {
                None => not_detected = !not_detected,
                Some(column) => match &condition[or_counter][and_counter].value {
                    None => return false,
                    Some(value) => {
                        if not_detected {
                            and_valid = !operate_condition(
                                &elements[*column],
                                value,
                                &condition[or_counter][and_counter].condition,
                            )
                        } else {
                            and_valid = operate_condition(
                                &elements[*column],
                                value,
                                &condition[or_counter][and_counter].condition,
                            );
                        }
                    }
                },
            }

            and_counter += 1;
        }

        or_valid = and_valid;

        or_counter += 1;
    }

    or_valid
}
