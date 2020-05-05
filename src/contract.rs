use cosmwasm::errors::Result;
use cosmwasm::traits::{Api, Extern, Storage};
use cosmwasm::types::{log, CanonicalAddr, Env, HumanAddr, Response, LogAttribute};

use cw_storage::{serialize, PrefixedStorage};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::types::*;
use crate::resolver::*;
use crate::constant::*;
use crate::errors::*;

pub fn init<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    _env: Env,
    _msg: InitMsg,
) -> Result<Response> {
    is_valid_name(&_msg.name)?;
    is_valid_symbol(&_msg.symbol)?;

    let mut config_store = PrefixedStorage::new(CONFIG, &mut deps.storage);
    let state = serialize(&State {
        name: _msg.name,
        symbol: _msg.symbol,
    })?;

    config_store.set(KEY_STATE, &state);

    Ok(Response::default())
}

pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    msg: HandleMsg,
) -> Result<Response> {
    match msg {
        HandleMsg::Transfer { recipient, token_id } => transfer(deps, env, &recipient, token_id),
        HandleMsg::TransferFrom { sender, recipient, token_id } => transfer_from(deps, env, &sender, &recipient, token_id),
        HandleMsg::Approve { recipient, token_id } => approve(deps, env, &recipient, token_id),
        HandleMsg::ApproveForAll { owner, recipient } => approve_for_all(deps, env, &owner, &recipient),
        HandleMsg::Mint {} => mint(deps, env),
    }
}

pub fn query<S: Storage, A: Api>(deps: &Extern<S, A>, msg: QueryMsg) -> Result<Vec<u8>> {
    match msg {
        QueryMsg::Balance { address } => balance(deps, &address),
        QueryMsg::Owner { token_id } => owner(deps, token_id),
        QueryMsg::Allowance { token_id } => allowance(deps, token_id),
    }
}

fn transfer<S: Storage, A: Api>(
        deps: &mut Extern<S, A>,
        env: Env,
        recipient: &HumanAddr,
        value: u64
) -> Result<Response> {
    let sender_address_raw = &env.message.signer;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    execute_transfer(deps, &sender_address_raw, &recipient_address_raw, &token_id)
}

fn transfer_from<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    _env: Env,
    sender: &HumanAddr,
    recipient: &HumanAddr,
    value: u64
) -> Result<Response> {
    let sender_address_raw = deps.api.canonical_address(sender)?;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    execute_transfer(deps, &sender_address_raw, &recipient_address_raw, &token_id)
}

fn execute_transfer<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    from: &CanonicalAddr,
    to: &CanonicalAddr,
    token_id: &TokenId,
) -> Result<Response> {
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
        log("token_id", &token_id.to_string()),
    ];
    Ok(response(logs))
}

fn update_owner_tokens_store<T: Storage>(
    store: &mut T,
    token_id: &TokenId,
    owner: &CanonicalAddr,
    received: bool
) -> Result<()> {
    let mut token_id_set = read_owner_tokens_store(store, &owner)?;
    if received {
        token_id_set.push(token_id.clone());
        write_owner_tokens_store(store, &owner, &token_id_set)?;
    } else {
        let mut new_token_id_set :Vec<TokenId> = Vec::new();
        for elm in token_id_set.clone().into_iter() {
            if token_id.eq(&elm) {
                continue;
            }
            new_token_id_set.push(elm);
        }
        write_owner_tokens_store(store, &owner, &new_token_id_set)?;
    }
    Ok(())
}

fn approve<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    recipient: &HumanAddr,
    value: u64
) -> Result<Response> {
    let owner = &env.message.signer;
    let recipient_address_raw = deps.api.canonical_address(recipient)?;
    let token_id = TokenId::new(value);

    // validation token owner
    validate_token_owner(&deps.storage, &token_id, &owner)?;

    write_token_approvals_store(&mut deps.storage, &token_id, &recipient_address_raw)?;

    let logs = vec![
        log("action", "transfer_from"),
        log("owner", deps.api.human_address(&owner)?.as_str()),
        log("recipient", recipient.as_str()),
        log("token_id", &token_id.to_string()),
    ];

    Ok(response(logs))
}

// TODO: implement
fn approve_for_all<S: Storage, A: Api>(
    _deps: &mut Extern<S, A>,
    _env: Env,
    _owner: &HumanAddr,
    _recipient: &HumanAddr,
) -> Result<Response> {
    unimplemented!();
}

fn mint<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
) -> Result<Response> {
    let owner = &env.message.signer;

    // generate token
    let new_token_id = make_token_id(&mut deps.storage)?;

    write_token_owner_store(&mut deps.storage, &new_token_id, &owner)?;

    let mut token_id_set = read_owner_tokens_store(&mut deps.storage, &owner)?;

    token_id_set.push(new_token_id.clone());
    write_minted_token_id_store(&mut deps.storage, &token_id_set)?;
    write_owner_tokens_store(&mut deps.storage, &owner, &token_id_set)?;

    // TODO: create private method
    let res = Response {
        messages: vec![],
        log: vec![
            log("action", "mint"),
            log("token_id", &new_token_id.to_string()),
        ],
        data: None,
    };
    Ok(res)
}

fn balance<S: Storage, A: Api>(deps: &Extern<S, A>, address: &HumanAddr) -> Result<Vec<u8>> {
    let key = deps.api.canonical_address(address)?;
    let res = read_owner_tokens_store(&deps.storage, &key)?;
    serialize(&res.len())
}

fn owner<S: Storage, A: Api>(deps: &Extern<S, A>, value: u64) -> Result<Vec<u8>> {
    let canonical_address = read_token_owner_store(&deps.storage, &TokenId::new(value))?;
    let res = match canonical_address {
        Some(record) => Some(deps.api.human_address(&record)?),
        _ => None,
    };
    serialize(&res)
}

fn allowance<S: Storage, A: Api>(deps: &Extern<S, A>, value: u64) -> Result<Vec<u8>> {
    let token_id = TokenId::new(value);
    let res = match read_token_approvals_store(&deps.storage, &token_id)? {
        Some(record) => Some(deps.api.human_address(&record)?),
        _ => None,
    };
    serialize(&res)
}

fn read_owner_tokens_store<T: Storage>(store: &T, owner: &CanonicalAddr) -> Result<Vec<TokenId>> {
    let token_list = match owner_tokens_resolver_read(store).may_load(&owner.as_slice())? {
        Some(record) => record,
        None => {
            let v: Vec<TokenId> = vec![];
            v
        }
    };

    Ok(token_list)
}

fn write_owner_tokens_store<T: Storage>(store: &mut T, owner: &CanonicalAddr, token_id_set: &Vec<TokenId>) -> Result<()> {
    owner_tokens_resolver(store).save(owner.as_slice(), token_id_set)?;
    Ok(())
}

fn read_token_owner_store<T: Storage>(store: &T, token_id: &TokenId) -> Result<Option<CanonicalAddr>> {
    token_owner_resolver_read(store).may_load(&serialize(&token_id)?)
}

fn write_token_owner_store<T: Storage>(store: &mut T, token_id: &TokenId, owner: &CanonicalAddr) -> Result<()> {
    token_owner_resolver(store).save(&serialize(&token_id)?, owner)?;
    Ok(())
}

fn read_minted_token_id_store<T: Storage>(store: &T) -> Result<Option<Vec<TokenId>>> {
    minted_token_id_resolver_read(store).may_load(b"minter")
}

fn write_minted_token_id_store<T: Storage>(store: &mut T, token_id_set: &Vec<TokenId>) -> Result<()> {
    minted_token_ids_resolver(store).save(b"minter",token_id_set)?;
    Ok(())
}

fn read_token_approvals_store<T: Storage>(store: &T, token_id: &TokenId) -> Result<Option<CanonicalAddr>> {
    token_approvals_resolver_read(store).may_load(&serialize(&token_id)?)
}

fn write_token_approvals_store<T: Storage>(store: &mut T, token_id: &TokenId, addr: &CanonicalAddr) -> Result<()> {
    token_approvals_resolver(store).save(&serialize(&token_id)?, addr)?;
    Ok(())
}

fn make_token_id<T: Storage>(store: &mut T) -> Result<TokenId> {
    let new_token_id = match get_current_token_id(store)? {
        Some(v) => v.as_u64() + 1,
        None => 0,
    };

    Ok(TokenId::new(new_token_id))
}

fn get_current_token_id<T: Storage>(store: &T) -> Result<Option<TokenId>> {
    let token_id_set = match read_minted_token_id_store(store)? {
        Some(record) => record,
        _ => return Ok(None),
    };

    let last_token_id = token_id_set.last().unwrap().clone();
    Ok(Some(last_token_id))
}

fn validate_token_id<T: Storage>(store: &T, token_id: &TokenId) -> Result<()> {
    let current_token_id = get_current_token_id(store)?;
    if current_token_id.unwrap().as_u64() < token_id.as_u64() {
        return err_token_not_exist();
    }
    Ok(())
}

fn validate_token_owner<T: Storage>(store: &T, token_id: &TokenId, addr: &CanonicalAddr) -> Result<()> {
    let token_owner = match read_token_owner_store(store, &token_id)? {
        Some(v) => v,
        _ => return err_token_owner_not_exist(),
    };

    if !token_owner.eq(addr) {
        return err_invalid_token_owner();
    }

    Ok(())
}

fn validate_allowance<T: Storage>(store: &T, token_id: &TokenId, addr: &CanonicalAddr) -> Result<()> {
    let token_approval = match read_token_approvals_store(store, &token_id)? {
        Some(v) => v,
        _ => return err_token_allowance_not_exist(),
    };
    if !token_approval.eq(&addr) {
        return err_invalid_token_allowance();
    }
    Ok(())
}

fn response(logs: Vec<LogAttribute>) -> Response {
    Response {messages: vec![], log: logs, data: None}
}

fn is_valid_name(name: &str) -> Result<()> {
    if name.chars().count() < 3 || name.chars().count() > 30 {
        return err_invalid_name();
    }
    Ok(())
}

fn is_valid_symbol(symbol: &str) -> Result<()> {
    if symbol.chars().count() < 3 || symbol.chars().count() > 30 {
        return err_invalid_symbol();
    }
    Ok(())
}
