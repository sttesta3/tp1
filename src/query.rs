#[derive(Debug)]

enum queryType {
    DELETE,
    INSERT,
    SELECT,
    UPDATE
}

struct query {
    operation:          queryType,
    tabla:              String,                 // DELETE, INSERT, SELECT, UPDATE
    selectores:         Option<Vec<String>>,    // INSERT, SELECT, UPDATE
    where_condition:    Option<String>,         // DELETE (siempre), SELECT (a veces), UPDATE (siempre) 
    order_by:           Option<String>,         // SELECT (a veces)
    values:             Option<String>          // INSERT y UPDATE (en update es set)
}

impl std::fmt::Display for query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.operation == queryType::DELETE {
            write!(f,"{}","DELETE")
        } else if self.operation == queryType::INSERT {
            write!(f,"{}","INSERT")
        } else if self.operation == queryType::SELECT {
            write!(f,"{}","SELECT")
        } else if self.operation == queryType::UPDATE {
            write!(f,"{}","UPDATE")
        }
    }
}