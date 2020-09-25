use cosmwasm_std::{
    log, to_binary, to_vec, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, LogAttribute, Querier, StdResult, Storage, Uint128,
};

use cosmwasm_storage::PrefixedStorage;
use std::ops::Add;

use crate::constant::*;
use crate::errors::*;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::store::*;
use crate::types::*;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    is_valid_name(&_msg.name)?;
    is_valid_symbol(&_msg.symbol)?;

    let mut config_store = PrefixedStorage::new(CONFIG, &mut deps.storage);
    let state = to_vec(&State {
        name: _msg.name,
        symbol: _msg.symbol,
    })?;

    config_store.set(KEY_STATE, &state);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Transfer {
            recipient,
            token_id,
        } => transfer(deps, env, &recipient, token_id),
        HandleMsg::TransferFrom {
            sender,
            recipient,
            token_id,
        } => transfer_from(deps, env, &sender, &recipient, token_id),
        HandleMsg::Approve {
            recipient,
            token_id,
        } => approve(deps, env, &recipient, token_id),
        HandleMsg::ApproveForAll { owner, recipient } => {
            approve_for_all(deps, env, &owner, &recipient)
        }
        HandleMsg::Mint {} => mint(deps, env),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => balance(deps, &address),
        QueryMsg::Owner { token_id } => owner(deps, token_id),
        QueryMsg::Allowance { token_id } => allowance(deps, token_id),
    }
}

fn transfer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    recipient: &HumanAddr,
    value: Uint128,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    execute_transfer(deps, &sender_address_raw, &recipient_address_raw, &token_id)
}

fn transfer_from<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    sender: &HumanAddr,
    recipient: &HumanAddr,
    value: Uint128,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(sender)?;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    execute_transfer(deps, &sender_address_raw, &recipient_address_raw, &token_id)
}

fn execute_transfer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    from: &CanonicalAddr,
    to: &CanonicalAddr,
    token_id: &TokenId,
) -> StdResult<HandleResponse> {
    // validation token
    validate_token_id(&deps.storage, &token_id)?;

    // validation token owner
    validate_token_owner(&deps.storage, &token_id, &from)?;

    // validation allowance
    validate_allowance(&deps.storage, &token_id, &to)?;

    /* update owner_tokens_store */
    // for from addr
    update_owner_tokens_store(&mut deps.storage, &token_id, &from, false)?;
    // for to addr
    update_owner_tokens_store(&mut deps.storage, &token_id, &to, true)?;

    // update token_owner_store
    write_token_owner_store(&mut deps.storage, &token_id, &to)?;

    let logs = vec![
        log("action", "transfer_from"),
        log("sender", deps.api.human_address(from)?.as_str()),
        log("recipient", deps.api.human_address(to)?.as_str()),
        log("token_id", &token_id.as_string()),
    ];
    Ok(response(logs))
}

fn update_owner_tokens_store<T: Storage>(
    store: &mut T,
    token_id: &TokenId,
    owner: &CanonicalAddr,
    received: bool,
) -> StdResult<()> {
    let mut token_id_set = read_owner_tokens_store(store, &owner)?;
    if received {
        token_id_set.push(token_id.clone());
        write_owner_tokens_store(store, &owner, token_id_set)?;
    } else {
        let mut new_token_id_set: Vec<TokenId> = Vec::new();
        for elm in token_id_set.into_iter() {
            if token_id.equal(&elm) {
                continue;
            }
            new_token_id_set.push(elm);
        }
        write_owner_tokens_store(store, &owner, new_token_id_set)?;
    }
    Ok(())
}

fn approve<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    recipient: &HumanAddr,
    value: Uint128,
) -> StdResult<HandleResponse> {
    let owner = deps.api.canonical_address(&env.message.sender)?;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    // validation token owner
    validate_token_owner(&deps.storage, &token_id, &owner)?;

    write_token_approvals_store(&mut deps.storage, &token_id, &recipient_address_raw)?;

    let logs = vec![
        log("action", "transfer_from"),
        log("owner", deps.api.human_address(&owner)?.as_str()),
        log("recipient", recipient.as_str()),
        log("token_id", &token_id.as_string()),
    ];

    Ok(response(logs))
}

// TODO: implement
fn approve_for_all<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _owner: &HumanAddr,
    _recipient: &HumanAddr,
) -> StdResult<HandleResponse> {
    unimplemented!();
}

fn mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let owner = deps.api.canonical_address(&env.message.sender)?;

    // generate token
    let new_token_id = make_token_id(&mut deps.storage)?;

    write_token_owner_store(&mut deps.storage, &new_token_id, &owner)?;

    let mut token_id_set = read_owner_tokens_store(&deps.storage, &owner)?;

    token_id_set.push(new_token_id.clone());
    write_minted_token_id_store(&mut deps.storage, token_id_set.clone())?;
    write_owner_tokens_store(&mut deps.storage, &owner, token_id_set)?;

    let logs = vec![
        log("action", "mint"),
        log("token_id", &new_token_id.as_string()),
    ];

    Ok(response(logs))
}

fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
) -> StdResult<Binary> {
    let key = deps.api.canonical_address(address)?;
    let res = read_owner_tokens_store(&deps.storage, &key)?;
    Ok(to_binary(&res.len())?)
}

fn owner<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    value: Uint128,
) -> StdResult<Binary> {
    let canonical_address = read_token_owner_store(&deps.storage, &TokenId::new(value))?;
    let res = match canonical_address {
        Some(record) => Some(deps.api.human_address(&record)?),
        _ => None,
    };
    Ok(to_binary(&res)?)
}

fn allowance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    value: Uint128,
) -> StdResult<Binary> {
    let token_id = TokenId::new(value);
    let res = match read_token_approvals_store(&deps.storage, &token_id)? {
        Some(record) => Some(deps.api.human_address(&record)?),
        _ => None,
    };
    Ok(to_binary(&res)?)
}

fn make_token_id<T: Storage>(store: &mut T) -> StdResult<TokenId> {
    let new_token_id = match get_current_token_id(store)? {
        Some(v) => v.as_u128().add(Uint128(1)),
        None => Uint128(0),
    };

    Ok(TokenId::new(new_token_id))
}

fn get_current_token_id<T: Storage>(store: &T) -> StdResult<Option<TokenId>> {
    let token_id_set = match read_minted_token_id_store(store)? {
        Some(record) => record,
        _ => return Ok(None),
    };

    let last_token_id = token_id_set.last().unwrap().clone();
    Ok(Some(last_token_id))
}

fn validate_token_id<T: Storage>(store: &T, token_id: &TokenId) -> StdResult<()> {
    let current_token_id = get_current_token_id(store)?;
    if current_token_id.unwrap().as_u128() < token_id.as_u128() {
        return err_token_not_exist();
    }
    Ok(())
}

fn validate_token_owner<T: Storage>(
    store: &T,
    token_id: &TokenId,
    addr: &CanonicalAddr,
) -> StdResult<()> {
    let token_owner = match read_token_owner_store(store, &token_id)? {
        Some(v) => v,
        _ => return err_token_owner_not_exist(),
    };

    if !token_owner.eq(addr) {
        return err_invalid_token_owner();
    }

    Ok(())
}

fn validate_allowance<T: Storage>(
    store: &T,
    token_id: &TokenId,
    addr: &CanonicalAddr,
) -> StdResult<()> {
    let token_approval = match read_token_approvals_store(store, &token_id)? {
        Some(v) => v,
        _ => return err_token_allowance_not_exist(),
    };
    if !token_approval.eq(&addr) {
        return err_invalid_token_allowance();
    }
    Ok(())
}

fn response(logs: Vec<LogAttribute>) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: logs,
        data: None,
    }
}

fn is_valid_name(name: &str) -> StdResult<()> {
    if name.chars().count() < 3 || name.chars().count() > 30 {
        return err_invalid_name();
    }
    Ok(())
}

fn is_valid_symbol(symbol: &str) -> StdResult<()> {
    if symbol.chars().count() < 3 || symbol.chars().count() > 30 {
        return err_invalid_symbol();
    }
    Ok(())
}
