use cosmwasm::traits::Storage;
use cosmwasm::types::CanonicalAddr;
use crate::constant::*;

use cw_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use crate::types::*;

pub fn owner_tokens_resolver<S: Storage>(storage: &mut S) -> Bucket<S, Vec<TokenId>> {
    bucket(OWNER_TOKENS, storage)
}

pub fn owner_tokens_resolver_read<S: Storage>(storage: &S) -> ReadonlyBucket<S, Vec<TokenId>> {
    bucket_read(OWNER_TOKENS, storage)
}

pub fn token_owner_resolver<S: Storage>(storage: &mut S) -> Bucket<S, CanonicalAddr> {
    bucket(TOKEN_OWNER, storage)
}

pub fn token_owner_resolver_read<S: Storage>(storage: &S) -> ReadonlyBucket<S, CanonicalAddr> {
    bucket_read(TOKEN_OWNER, storage)
}

pub fn token_approvals_resolver<S: Storage>(storage: &mut S) -> Bucket<S, CanonicalAddr> {
    bucket(TOKEN_APPROVALS, storage)
}

pub fn token_approvals_resolver_read<S: Storage>(storage: &S) -> ReadonlyBucket<S, CanonicalAddr> {
    bucket_read(TOKEN_APPROVALS, storage)
}

pub fn minted_token_ids_resolver<S: Storage>(storage: &mut S) -> Bucket<S, Vec<TokenId>> {
    bucket(MINTED_TOKEN_ID, storage)
}

pub fn minted_token_id_resolver_read<S: Storage>(storage: &S) -> ReadonlyBucket<S, Vec<TokenId>> {
    bucket_read(MINTED_TOKEN_ID, storage)
}
