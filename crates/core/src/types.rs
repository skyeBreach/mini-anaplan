use std::sync::Arc;

use rand::RngExt;

const ID_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Id(Arc<str>);

impl Id {
    // TODO: Move to not allocating
    pub fn new(prefix: impl AsRef<str>) -> Self {
        let random: String = rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(ID_LENGTH)
            .map(char::from)
            .collect();

        Id(Arc::from(format!("{}_{}", prefix.as_ref(), random)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
