pub mod condition_type;
use condition_type::BooleanOperator;
use condition_type::ConditionOperator;

pub struct Condition {
    pub condition:  ConditionOperator,
    pub v1:       Option<String>,
    pub v2:       Option<String>,
}

pub struct ComplexCondition {
    pub operator:   BooleanOperator,
    pub cond1:      Condition,
    pub nest_cond:  Box<ComplexCondition>   // Linked list of complex conditions 
}

pub fn build_condition(v1: String, v2: String, condition: ConditionOperator) -> Condition {
    return Condition {
        condition:  condition,
        v1:         Some(v1),
        v2:         Some(v2),
    }
}

pub fn build_complex_condition(op: BooleanOperator,condition: Condition, comp: Box<ComplexCondition>) -> ComplexCondition {
    return Condition {
        operator:   op,
        cond1:      condition,
        nest_cond:  comp   // Linked list of complex conditions 
    }
}