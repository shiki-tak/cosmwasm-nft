use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// INIT="{\"name\":\"wasm-cosmwasm_nft\", \"symbol\":\"WSM\"}"
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // TRANFER="{\"transfer\":{\"recipient\": \"$(wasmcli keys show fred -a)\", \"token_id\": 0}}"
    // wasmcli tx wasm execute $CONTRACT $TRANFER --from validator -y
    Transfer {
        recipient: String,
        token_id: Uint128,
    },
    // TRANFERFROM="{\"transfer_from\":{\"owner\": \"$(wasmcli keys show validator -a)\", \"recipient\": \"$(wasmcli keys show fred -a)\", \"token_id\": 0}}"
    // wasmcli tx wasm execute $CONTRACT $TRANFERFROM --from validator -y
    TransferFrom {
        sender: String,
        recipient: String,
        token_id: Uint128,
    },
    // APPROVE="{\"approve\":{\"recipient\": \"$(wasmcli keys show fred -a)\", \"token_id\": 0}}"
    // wasmcli tx wasm execute $CONTRACT $APPROVE --from validator -y
    Approve {
        recipient: String,
        token_id: Uint128,
    },
    ApproveForAll {
        owner: String,
        recipient: String,
    },
    // MINT="{\"mint\":{}}"
    // wasmcli tx wasm execute $CONTRACT $MINT --from validator -y
    Mint {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // BALANCE="{\"balance\":{\"address\": \"$(wasmcli keys show fred -a)\"}}"
    // wasmcli query wasm contract-state smart $CONTRACT $BALANCE
    Balance { address: String },
    // OWNER="{\"owner\":{\"token_id\": 0}}"
    // wasmcli query wasm contract-state smart $CONTRACT $OWNER
    Owner { token_id: Uint128 },
    // ALLOWANCE="{\"allowance\":{\"token_id\": 0}}"
    // wasmcli query wasm contract-state smart $CONTRACT $ALLOWANCE
    Allowance { token_id: Uint128 },
}
