use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier(pub Uuid);

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_unique_identifiers() {
        let a = Identifier::new();
        let b = Identifier::new();

        assert_ne!(a, b);
    }
}
