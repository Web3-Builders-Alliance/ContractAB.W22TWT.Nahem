use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub contract_id: u64,
    pub contract_addr: Option<String>,
}

pub const STATE: Item<State> = Item::new("state");
