use cosmwasm_std::{DepsMut, Response};

use crate::{state::STATE, ContractError};

pub fn redirect_funds(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |state| -> Result<_, ContractError> {
        // state.latest_contract_id += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "redirect_funds"))
}
