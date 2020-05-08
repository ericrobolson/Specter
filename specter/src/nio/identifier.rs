use super::*;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub itype: Option<String>,
    pub id: String,
}

impl Identifier {
    pub fn new(id: String) -> Self {
        return Self {
            id: id.to_lowercase(),
            itype: None,
        };
    }
}
