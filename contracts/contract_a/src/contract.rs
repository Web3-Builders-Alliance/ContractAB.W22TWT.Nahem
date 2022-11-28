use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, InstantiateMsgB, QueryMsg};
use crate::query;
use crate::state::{State, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::{parse_reply_execute_data, parse_reply_instantiate_data};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_REPLY_ID: u64 = 0;
const REDIRECT_FUNDS_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        admin: info.sender.clone(),
        contract_id: msg.contract_id,
        contract_addr: None,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InstantiateNewContract {} => instantiate_new_contract(deps),
        ExecuteMsg::RedirectFunds { to_address, amount } => {
            redirect_funds(deps, to_address, amount)
        }
    }
}

pub fn instantiate_new_contract(deps: DepsMut) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let instantiate_msg = WasmMsg::Instantiate {
        admin: None,
        code_id: state.contract_id,
        msg: to_binary(&InstantiateMsgB {})?,
        funds: vec![],
        label: "contract b".to_string(),
    };
    let reply_msg = SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_REPLY_ID);
    Ok(Response::new().add_submessage(reply_msg))
}

pub fn redirect_funds(
    deps: DepsMut,
    address: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let val_addr = deps.api.addr_validate(&address)?;

    let bank_msg = BankMsg::Send {
        to_address: val_addr.to_string(),
        amount: coins(amount, "ujuno".to_string()),
    };

    let sub_msg = SubMsg::reply_on_success(bank_msg, REDIRECT_FUNDS_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "redirect_funds")
        .add_attribute("to_address", address)
        .add_attribute("amount", amount.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        REDIRECT_FUNDS_ID => handle_redirect_funds_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

pub fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg)
        .map_err(|_| StdError::generic_err("error parsing instantiate reply"))?;
    let validated_addr = deps.api.addr_validate(&res.contract_address)?;
    let mut state = STATE.load(deps.storage)?;
    state.contract_id = INSTANTIATE_REPLY_ID;
    state.contract_addr = Some(validated_addr.to_string());
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

fn handle_redirect_funds_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let _res = parse_reply_execute_data(msg)
        .map_err(|_| StdError::generic_err("error parsing redirect funds reply"))?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_binary(&query::get_admin(deps)?),
    }
}
