pub mod constant;
pub mod contract;
pub mod errors;
pub mod msg;
pub mod resolver;
pub mod store;
pub mod types;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
