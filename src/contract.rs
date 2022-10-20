#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, to_vec, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Storage, Uint128,
};

use cosmwasm_storage::PrefixedStorage;
use std::ops::Add;

use crate::constant::*;
use crate::errors::ContractError;
use crate::msg::{InstantiateMsg,ExecuteMsg, QueryMsg};
use crate::store::*;
use crate::types::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    is_valid_name(&msg.name)?;
    is_valid_symbol(&msg.symbol)?;

    let mut config_store = PrefixedStorage::new(deps.storage, CONFIG);
    let state = to_vec(&State {
        name: msg.name,
        symbol: msg.symbol,
    })?;

    config_store.set(KEY_STATE, &state);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer {
            recipient,
            token_id,
        } => transfer(deps, env, info.sender.to_string(), recipient, token_id),
        ExecuteMsg::TransferFrom {
            sender,
            recipient,
            token_id,
        } => transfer_from(deps, env, sender, recipient, token_id),
        ExecuteMsg::Approve {
            recipient,
            token_id,
        } => approve(deps, env, info.sender.to_string(), recipient, token_id),
        ExecuteMsg::ApproveForAll { owner, recipient } => {
            approve_for_all(deps, env, owner, recipient)
        }
        ExecuteMsg::Mint {} => mint(deps, env, info.sender.to_string()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => balance(deps, address),
        QueryMsg::Owner { token_id } => owner(deps, token_id),
        QueryMsg::Allowance { token_id } => allowance(deps, token_id),
    }
}

fn transfer(
    deps: DepsMut,
    _env: Env,
    sender: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);

    execute_transfer(deps, sender, recipient, &token_id)
}

fn transfer_from(
    deps: DepsMut,
    _env: Env,
    sender: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);
    
    // validation allowance
    validate_allowance(deps.storage, &token_id, recipient.clone())?;

    execute_transfer(deps, sender, recipient, &token_id)
}

fn execute_transfer(
    deps: DepsMut,
    from: String,
    to: String,
    token_id: &TokenId,
) -> Result<Response, ContractError> {
    // validation token
    validate_token_id(deps.storage, &token_id)?;

    // validation token owner
    validate_token_owner(deps.storage, &token_id, from.clone())?;

    /* update owner_tokens_store */
    // for from addr
    update_owner_tokens_store(deps.storage, &token_id, from.clone(), false)?;
    // for to addr
    update_owner_tokens_store(deps.storage, &token_id, to.clone(), true)?;

    // update token_owner_store
    write_token_owner_store(deps.storage, &token_id, to.clone())?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "transfer_from"),
            attr("sender", from),
            attr("recipient", to),
            attr("token_id", &token_id.as_string()),
        ],
        data: None,
    };
    Ok(res)
}

fn update_owner_tokens_store(
    store: &mut dyn Storage,
    token_id: &TokenId,
    owner: String,
    received: bool,
) -> StdResult<()> {
    let mut token_id_set = read_owner_tokens_store(store, owner.clone())?;
    if received {
        token_id_set.push(token_id.clone());
        write_owner_tokens_store(store, owner.clone(), token_id_set)?;
    } else {
        let mut new_token_id_set: Vec<TokenId> = Vec::new();
        for elm in token_id_set.into_iter() {
            if token_id.equal(&elm) {
                continue;
            }
            new_token_id_set.push(elm);
        }
        write_owner_tokens_store(store, owner, new_token_id_set)?;
    }
    Ok(())
}

fn approve(
    deps: DepsMut,
    _env: Env,
    owner: String,
    recipient: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let token_id = TokenId::new(value);

    // validation token owner
    validate_token_owner(deps.storage, &token_id, owner.clone())?;

    write_token_approvals_store(deps.storage, &token_id, recipient.clone())?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "approve"),
            attr("owner", owner),
            attr("recipient", recipient.as_str()),
            attr("token_id", &token_id.as_string()),
            ],
        data: None,
    };

    Ok(res)
}

// TODO: implement
fn approve_for_all(
    _deps: DepsMut,
    _env: Env,
    _owner: String,
    _recipient: String,
) -> Result<Response, ContractError> {
    unimplemented!();
}

fn mint(
    deps: DepsMut,
    _env: Env,
    owner: String,
) -> Result<Response, ContractError> {
    // generate token
    let new_token_id = make_token_id(deps.storage)?;

    write_token_owner_store(deps.storage, &new_token_id, owner.clone())?;

    let mut token_id_set = read_owner_tokens_store(deps.storage, owner.clone())?;

    token_id_set.push(new_token_id.clone());
    write_minted_token_id_store(deps.storage, token_id_set.clone())?;
    write_owner_tokens_store(deps.storage, owner.clone(), token_id_set)?;

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "mint"),
            attr("token_id", &new_token_id.as_string()),
            ],
        data: None,
    };

    Ok(res)
}

fn balance(
    deps: Deps,
    address: String,
) -> StdResult<Binary> {
    let res = read_owner_tokens_store(deps.storage, address)?;
    Ok(to_binary(&res.len())?)
}

fn owner(
    deps: Deps,
    value: Uint128,
) -> StdResult<Binary> {
    let address = read_token_owner_store(deps.storage, &TokenId::new(value))?;
    
    Ok(to_binary(&address)?)
}

fn allowance(
    deps: Deps,
    value: Uint128,
) -> StdResult<Binary> {
    let token_id = TokenId::new(value);
    let res = read_token_approvals_store(deps.storage, &token_id)?;

    Ok(to_binary(&res)?)
}

fn make_token_id(store: &mut dyn Storage) -> StdResult<TokenId> {
    let new_token_id = match get_current_token_id(store)? {
        Some(v) => v.as_u128().add(Uint128(1)),
        None => Uint128(0),
    };

    Ok(TokenId::new(new_token_id))
}

fn get_current_token_id(store: &dyn Storage) -> StdResult<Option<TokenId>> {
    let token_id_set = match read_minted_token_id_store(store)? {
        Some(record) => record,
        _ => return Ok(None),
    };

    let last_token_id = token_id_set.last().unwrap().clone();
    Ok(Some(last_token_id))
}

fn validate_token_id(store: &dyn Storage, token_id: &TokenId) -> Result<(), ContractError> {
    let current_token_id = get_current_token_id(store)?;
    if current_token_id.unwrap().as_u128() < token_id.as_u128() {
        return Err(ContractError::NotExistToken {});
    }
    Ok(())
}

fn validate_token_owner(
    store: &dyn Storage,
    token_id: &TokenId,
    addr: String,
) -> Result<(), ContractError> {
    let token_owner = match read_token_owner_store(store, &token_id)? {
        Some(v) => v,
        _ => return Err(ContractError::NotExistTokenOwner {}),
    };

    if !token_owner.eq(&addr) {
        return Err(ContractError::InvalidTokenOwner {});
    }

    Ok(())
}

fn validate_allowance(
    store: &dyn Storage,
    token_id: &TokenId,
    addr: String,
) -> Result<(), ContractError> {
    let token_approval = match read_token_approvals_store(store, &token_id)? {
        Some(v) => v,
        _ => return Err(ContractError::NotExistTokenAllowance {}),
    };
    if !token_approval.eq(&addr) {
        return Err(ContractError::InvalidTokenAllowance {});
    }
    Ok(())
}

fn is_valid_name(name: &str) -> Result<(), ContractError> {
    if name.chars().count() < 3 || name.chars().count() > 30 {
        return Err(ContractError::InvalidNameFormat {});
    }
    Ok(())
}

fn is_valid_symbol(symbol: &str) -> Result<(), ContractError> {
    if symbol.chars().count() < 3 || symbol.chars().count() > 30 {
        return Err(ContractError::InvalidSymbolFormat {});
    }
    Ok(())
}
