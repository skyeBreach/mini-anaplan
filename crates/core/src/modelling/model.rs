use anyhow::Result;

use crate::types::Id;

#[derive(Debug, Clone)]
pub struct Model {
    pub id: Id,
    pub name: String,
}

impl Model {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            id: Id::new("mdl"),
            name: name.to_string(),
        })
    }
}
