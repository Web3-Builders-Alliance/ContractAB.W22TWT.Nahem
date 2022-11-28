use cosmwasm_std::{Addr, Deps, StdResult};

use crate::msg::{GetAdminResponse, GetContractFundsResponse};
use crate::state::CONFIG;

pub fn get_admin(deps: Deps) -> StdResult<GetAdminResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(GetAdminResponse { admin: state.admin })
}

pub fn get_contract_funds(deps: Deps, contract_addr: Addr) -> StdResult<GetContractFundsResponse> {
    let funds = deps.querier.query_all_balances(contract_addr)?;
    Ok(GetContractFundsResponse { funds: funds })
}
