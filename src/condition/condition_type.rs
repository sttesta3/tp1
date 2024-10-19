#[derive(Debug)]
pub enum ConditionOperator {
    Minor,
    MinorEqual,
    Equal,
    Higher,
    HigherEqual,
}

#[derive(Debug)]
pub enum BooleanOperator {
    AND,
    OR,
    NOT, // NOT.         Left child
    I,   // identity.
}

/*
impl PartialEq for BooleanOperator {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Eq for BooleanOperator {}
*/
