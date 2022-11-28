use cosmwasm_std::{Deps, StdResult};

use crate::{msg::GetAdminResponse, state::STATE};

pub fn get_admin(deps: Deps) -> StdResult<GetAdminResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetAdminResponse { admin: state.admin })
}
