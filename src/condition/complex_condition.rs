use std::borrow::BorrowMut;

use crate::{libs::error::WHERE_MAL_FORMATEADO, query::Query};
use super::{condition_type::{self, ConditionOperator}, BooleanOperator, Condition};

pub struct ComplexCondition {
    // Tree of complex conditions 
    pub operator:       BooleanOperator,
    pub left_cond:      Option<Box<ComplexCondition>>,   // Oldest condition ( first to be found )
    pub left_simple:    Option<Condition>,
    pub right_cond:     Option<Box<ComplexCondition>>,   // Newest condition ( last to be found )
    pub right_simple:   Option<Condition>,
}

pub fn build_simple_condition(left: Option<Condition>) -> ComplexCondition {
    ComplexCondition {
        operator:       BooleanOperator::I,
        left_simple:    left,

        left_cond:      None,
        right_simple:   None,
        right_cond:     None,
    }
}

pub fn build_complex_condition(op: BooleanOperator, left: Option<Box<ComplexCondition>>, right: Option<Box<ComplexCondition>>) -> ComplexCondition {
    ComplexCondition {
        operator:       op,
        left_cond:      left,
        right_cond:     right,

        left_simple:    None,
        right_simple:   None
    }
} 

pub fn tree_check(root: &ComplexCondition) -> bool {
    // In Order valid check ( no None leafs )
    match &root.operator {
        BooleanOperator::OR     => tree_check_or_and(&root),
        BooleanOperator::AND    => tree_check_or_and(&root),
        BooleanOperator::NOT    => tree_check_not_i(&root),
        BooleanOperator::I      => tree_check_not_i(&root)
    }
}

fn tree_check_or_and(root: &ComplexCondition) -> bool {
    match &root.left_cond {
        Some(left) => {
            match &root.right_cond {
                Some(right) => return tree_check(&*left) && tree_check(&*right),
                None => return false
            }
        },
        None => return false
    }
}

fn tree_check_not_i(root: &ComplexCondition) -> bool {
    match &root.left_cond {
        Some(left) => {
            match root.right_cond {
                Some(_) => return false,
                None => return tree_check(&*left)
            }
        },
        None => return false
    }
}

pub fn add_node_to_tree(mut new_node: ComplexCondition, root: &mut ComplexCondition) -> Result<ComplexCondition,u32> {
    // Pre:  Previous root and new node
    // Post: If success new root, else Error
    if check_precedence(&new_node.operator, &root.operator) {           // IM new root
        new_node.left_cond =  Some(Box::new(*root));
        Ok(new_node)
    } else if root.left_cond.is_none() {             // IM RIGHT SON 
        Err(WHERE_MAL_FORMATEADO)
    } else {
        match root.right_cond.as_deref() {
            Some(mut x) => add_node_to_tree(new_node, x.borrow_mut()),
            None => {
                root.right_cond = Some(Box::new(new_node));
                return Ok(*root)
            }
        }
    }
}

fn check_valid_args_for_basic_condition(args: &Vec<String>, start_position: usize) -> bool {
    if start_position + 3 > args.len() {
        false
    } else {
        let first = &args[start_position];
        let second = &args[start_position + 1];
        let third = &args[start_position + 2];

        if first.eq("AND") || first.eq("OR") || first.eq("NOT") {
            false
        } else if second.eq("AND") || second.eq("OR") || second.eq("NOT") {
            false 
        } else if ! ( second.eq(">") || second.eq(">=") || second.eq("=") || second.eq("<") || second.eq("<=") ) {
            false
        } else if third.eq("AND") || third.eq("OR") || third.eq("NOT") {
            false
        } else {
            true 
        }
    }  
}

pub fn check_precedence(op1: &BooleanOperator, op2: &BooleanOperator) -> bool {
    // Pre:  Operators
    // Post: True if it's higher on the tree, false if not 
    match op1 {
        BooleanOperator::OR => true,
        BooleanOperator::AND => {
            if *op2 == BooleanOperator::OR {
                false
            } else {
                true
            }
        },
        BooleanOperator::NOT => {
            if *op2 == BooleanOperator::OR || *op2 == BooleanOperator::AND {
                false
            } else {
                true
            }
        },
        BooleanOperator::I => false,
    }
}