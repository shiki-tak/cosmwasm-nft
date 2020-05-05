use cosmwasm::errors::{contract_err, Result};

pub fn err_invalid_name() -> Result<()> {
    contract_err("Err invalid name format")
}

pub fn err_invalid_symbol() -> Result<()> {
    contract_err("Err invalid symbol format")
}

pub fn err_invalid_token_owner() -> Result<()> {
    contract_err("Err invalid token owner")
}

pub fn err_token_not_exist() -> Result<()> {
    contract_err("Err token not exist")
}

pub fn err_token_owner_not_exist() -> Result<()> {
    contract_err("Err token owner not exist")
}

pub fn err_token_allowance_not_exist() -> Result<()> {
    contract_err("Err token allowance not exist")
}

pub fn err_invalid_token_allowance() -> Result<()> {
    contract_err("Err invalid token allowance")
}
