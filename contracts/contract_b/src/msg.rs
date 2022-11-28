use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw { to_address: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},
    #[returns(GetContractFundsResponse)]
    GetContractFunds { contract_addr: Addr },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Addr,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetContractFundsResponse {
    pub funds: Vec<Coin>,
}
