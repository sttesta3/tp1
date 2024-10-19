use core::fmt;
pub enum QueryType {
    DELETE,
    INSERT,
    SELECT,
    UPDATE,
}

impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryType::DELETE => write!(f, "DELETE"),
            QueryType::INSERT => write!(f, "INSERT"),
            QueryType::SELECT => write!(f, "SELECT"),
            QueryType::UPDATE => write!(f, "UPDATE"),
        }
    }
}
