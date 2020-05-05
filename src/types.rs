use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenId(u64);

impl TokenId {
    pub fn new(v :u64) -> Self {
        TokenId(v)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
