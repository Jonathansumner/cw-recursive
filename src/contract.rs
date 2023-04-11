#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::msg::ExecuteMsg::Recurse;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:recursive-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let rec = Recurse { code_id: msg.code_id, depth: msg.depth-1 };
    let exec = SubMsg::new(WasmMsg::Execute {
        contract_addr: _env.contract.address.to_string(),
        msg: to_binary(&rec)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.clone().sender)
        .add_submessage(exec))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Recurse {code_id, depth} => execute::recurse(code_id, depth),
    }
}

pub mod execute {
    use super::*;

    pub fn recurse(code_id: u64, depth: u32) -> Result<Response, ContractError> {

        let msg = InstantiateMsg { depth, code_id };

        let exec = SubMsg::new(WasmMsg::Instantiate {
            admin: None,
            msg: to_binary(&msg)?,
            funds: vec![],
            code_id,
            label: "Recurse".to_string(),
        });

        let mut res = Response::new()
            .add_attribute("action", "recurse")
            .add_attribute("depth", depth.to_string());

        if depth >= 1 {res = res.add_submessage(exec);}

        Ok(res)
    }
}

