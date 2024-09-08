pub mod condition_type;
use std::ops::RemAssign;
use std::ptr::null;

use condition_type::BooleanOperator;
use condition_type::ConditionOperator;

pub struct Condition {
    pub condition:  ConditionOperator,
    pub v1:       Option<String>,
    pub v2:       Option<String>,
}

pub struct ComplexCondition {
    // Tree of complex conditions 
    pub operator:       Option<BooleanOperator>,
    pub condition:      Option<BooleanOperator>,
    pub left_cond:      Option<Box<ComplexCondition>>,  // Oldest condition ( first to be found )
    pub parent:         Option<Box<ComplexCondition>>,  // Newest condition ( last to be found )
    pub right_cond:     Option<Box<ComplexCondition>>   // Newest condition ( last to be found )
}

pub fn build_condition(v1: String, v2: String, condition: ConditionOperator) -> Condition {
    return Condition {
        condition:  condition,
        v1:         Some(v1),
        v2:         Some(v2),
    }
}

pub fn build_empty_complex_condition() -> ComplexCondition {
    return ComplexCondition {
        operator:       None,
        condition:      None,
        left_cond:      None,
        right_cond:     None,
        parent:         None,
    }
}

pub fn tree_check(root: ComplexCondition) -> bool {
    match &root.operator {
        Some(op) => {
            match op {
                BooleanOperator::OR  => tree_check_or_and(root),
                BooleanOperator::AND => tree_check_or_and(root),
                BooleanOperator::NOT => tree_check_not(root)
            }
        },
        None => return true
    }
}

fn tree_check_or_and(root: ComplexCondition) -> bool {
    match root.left_cond {
        Some(left) => {
            match root.right_cond {
                Some(right) => return tree_check(*left) && tree_check(*right),
                None => return false
            }
        },
        None => return false
    }
}

fn tree_check_not(root: ComplexCondition) -> bool {
    match root.left_cond {
        Some(left) => {
            match root.right_cond {
                Some(_) => return false,
                None => return tree_check(*left)
            }
        },
        None => return false
    }
}
