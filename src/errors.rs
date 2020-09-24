use cosmwasm_std::{
    StdError, StdResult,
};

pub fn err_invalid_name() -> StdResult<()> {
    Err(StdError::generic_err("Err invalid name format"))
}

pub fn err_invalid_symbol() -> StdResult<()> {
    Err(StdError::generic_err("Err invalid symbol format"))
}

pub fn err_invalid_token_owner() -> StdResult<()> {
    Err(StdError::generic_err("Err invalid token owner"))
}

pub fn err_token_not_exist() -> StdResult<()> {
    Err(StdError::generic_err("Err token not exist"))
}

pub fn err_token_owner_not_exist() -> StdResult<()> {
    Err(StdError::generic_err("Err token owner not exist"))
}

pub fn err_token_allowance_not_exist() -> StdResult<()> {
    Err(StdError::generic_err("Err token allowance not exist"))
}

pub fn err_invalid_token_allowance() -> StdResult<()> {
    Err(StdError::generic_err("Err invalid token allowance"))
}
