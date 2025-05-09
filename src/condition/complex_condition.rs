use crate::query::Query;
use crate::libs::error;
use super::BooleanOperator;

pub struct ComplexCondition {
    // Tree of complex conditions 
    pub operator:       Option<BooleanOperator>,
    pub condition:      Option<BooleanOperator>,
    pub left_cond:      Option<Box<ComplexCondition>>,  // Oldest condition ( first to be found )
    pub right_cond:     Option<Box<ComplexCondition>>   // Newest condition ( last to be found )
}

pub fn build_empty_complex_condition() -> ComplexCondition {
    return ComplexCondition {
        operator:       None,
        condition:      None,
        left_cond:      None,
        right_cond:     None
    }
}

pub fn build_complex_condition(operator: BooleanOperator, left: Option<Box<ComplexCondition>>, right: Option<Box<ComplexCondition>>) -> ComplexCondition {
    return ComplexCondition {
        operator:       Some(operator),
        condition:      None,
        left_cond:      left,
        right_cond:     right
    }
} 

pub fn tree_check(root: ComplexCondition) -> bool {
    // In Order valid check ( no None leafs )
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

pub fn add_node_to_tree(args: &Vec<String>, start_position: &mut usize, root: ComplexCondition) -> Result<ComplexCondition,u32> {
    if *start_position == args.len() || args[*start_position].eq("ORDER") {
        if tree_check(root) {
            Ok(root)
        } else {
            Err(error::WHERE_MAL_FORMATEADO)
        }
    } else if args[*start_position].eq("OR") {
        add_node_to_tree_or(args, &mut start_position, root)
    } else if args[*start_position].eq("AND") {
        add_node_to_tree_and(args, &mut start_position, root)
    } else if args[*start_position].eq("NOT") {
        add_node_to_tree_not(args, &mut start_position, root)
    } else if check_valid_args_for_basic_condition(args, *start_position) {
        match root.operator {
            Some(op) => {

            },
            None => return Err(error::WHERE_MAL_FORMATEADO) // C1 C2 
        }        
    } else {
        Err(error::WHERE_MAL_FORMATEADO)
    }
}

fn add_node_to_tree_or(args: &Vec<String>, start_position: &mut usize, root: ComplexCondition) -> Result<ComplexCondition,u32> {
    let new_root = build_complex_condition(BooleanOperator::OR, Some(Box::new(root)), None );
    *start_position += 1;
    add_node_to_tree(args, start_position, new_root)
}

fn add_node_to_tree_and(args: &Vec<String>, start_position: &mut usize, root: ComplexCondition) -> Result<ComplexCondition,u32> {
    match root.operator {
        Some(op) => {
            match op {
                BooleanOperator::OR  => {
                    match root.left_cond {
                        Some(x) => {},
                        None => return Err(error::WHERE_MAL_FORMATEADO)
                    }
                },
                BooleanOperator::AND => {
                    let new_root = build_complex_condition(BooleanOperator::AND, Some(Box::new(root)), None );
                    *start_position += 1;
                    add_node_to_tree(args, start_position , new_root)        
                },
                BooleanOperator::NOT => {
                    let new_root = build_complex_condition(BooleanOperator::AND, Some(Box::new(root)), None );
                    *start_position += 1;
                    add_node_to_tree(args, start_position, new_root)        
                },
            }
        },
        None => {
            let new_root = build_complex_condition(BooleanOperator::AND, Some(Box::new(root)), None );
            *start_position += 1;
            add_node_to_tree(args, start_position, new_root)
        }
    }
}

fn add_node_to_tree_not(args: &Vec<String>, start_position: &mut usize, root: ComplexCondition) -> Result<ComplexCondition,u32> {

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

pub fn get_where_columns(query: &Query, vec: Vec<String>, node: &ComplexCondition) -> Vec<String> {
    match node.operator {
        Some(_) => {},
        None => {
            match node.condition {
                Some(_) => {},
                None => return vec
            }
        }
    }
}