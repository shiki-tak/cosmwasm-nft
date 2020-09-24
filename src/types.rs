use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_vec, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenId(Uint128);

impl TokenId {
    pub fn new(v :Uint128) -> Self {
        TokenId(v)
    }

    pub fn as_u128(&self) -> Uint128 {
        self.0
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        to_vec(&self.0).unwrap()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
