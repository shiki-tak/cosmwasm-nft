use cosmwasm_std::{CanonicalAddr, StdResult, Storage};

use crate::resolver::*;
use crate::types::TokenId;

pub fn read_owner_tokens_store<T: Storage>(
    store: &T,
    owner: &CanonicalAddr,
) -> StdResult<Vec<TokenId>> {
    let token_list = match owner_tokens_resolver_read(store).may_load(&owner.as_slice())? {
        Some(record) => record,
        None => {
            let v: Vec<TokenId> = vec![];
            v
        }
    };
    Ok(token_list)
}

pub fn write_owner_tokens_store<T: Storage>(
    store: &mut T,
    owner: &CanonicalAddr,
    token_id_set: &Vec<TokenId>,
) -> StdResult<()> {
    owner_tokens_resolver(store).save(owner.as_slice(), token_id_set)?;
    Ok(())
}

pub fn read_token_owner_store<T: Storage>(
    store: &T,
    token_id: &TokenId,
) -> StdResult<Option<CanonicalAddr>> {
    token_owner_resolver_read(store).may_load(&token_id.as_bytes())
}

pub fn write_token_owner_store<T: Storage>(
    store: &mut T,
    token_id: &TokenId,
    owner: &CanonicalAddr,
) -> StdResult<()> {
    token_owner_resolver(store).save(&token_id.as_bytes(), owner)?;
    Ok(())
}

pub fn read_minted_token_id_store<T: Storage>(store: &T) -> StdResult<Option<Vec<TokenId>>> {
    minted_token_id_resolver_read(store).may_load(b"minter")
}

pub fn write_minted_token_id_store<T: Storage>(
    store: &mut T,
    token_id_set: &Vec<TokenId>,
) -> StdResult<()> {
    minted_token_ids_resolver(store).save(b"minter", token_id_set)?;
    Ok(())
}

pub fn read_token_approvals_store<T: Storage>(
    store: &T,
    token_id: &TokenId,
) -> StdResult<Option<CanonicalAddr>> {
    token_approvals_resolver_read(store).may_load(&token_id.as_bytes())
}

pub fn write_token_approvals_store<T: Storage>(
    store: &mut T,
    token_id: &TokenId,
    addr: &CanonicalAddr,
) -> StdResult<()> {
    token_approvals_resolver(store).save(&token_id.as_bytes(), addr)?;
    Ok(())
}
