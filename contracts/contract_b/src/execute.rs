use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response};

use crate::{state::CONFIG, ContractError};

pub fn redirect_funds(deps: DepsMut) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |config| -> Result<_, ContractError> {
        // state.latest_contract_id += 1;
        Ok(config)
    })?;

    Ok(Response::new().add_attribute("action", "redirect_funds"))
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to_address: Option<String>,
) -> Result<Response, ContractError> {
    // 1st: check that the contract has funds
    let funds = deps.querier.query_all_balances(env.contract.address)?;
    if funds.is_empty() {
        return Err(ContractError::NoFundsFound {});
    }

    // Q: is it better to use query::get_admin(deps)?
    // 2nd: check that only the contract admin can withdraw funds
    let admin = CONFIG.load(deps.storage)?.admin;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // 3rd: set recipient to validated address sent or to contract admin if None.
    let recipient = match to_address {
        Some(value) => deps.api.addr_validate(&value)?,
        None => admin,
    };

    // 4th: create bank message with validated recipient and all funds in the contract
    let bank_msg: BankMsg = BankMsg::Send {
        to_address: recipient.to_string(),
        amount: funds,
    };

    // 5th: return a response with a bank message that sends all funds to the recipient
    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw_funds")
        .add_attribute("recipient", recipient.to_string()))
}
