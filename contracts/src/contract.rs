use crate::error::ContractError;
use crate::msg::QueryMsg;
use crate::verifier::verify_proof;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult
    ,
};

// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::VerifyProof {
            proof, vk
        } => {
            verify_proof(&proof, &vk)?;
            Ok(to_json_binary(&())?)
        }
    }
}

pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::QueryMsg;
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use std::fs;

    #[test]
    fn test() {
        let mut app = App::default();
        let sender = Addr::unchecked("sender");

        let code = ContractWrapper::new(
            execute,
            instantiate,
            query,
        );
        let code_id = app.store_code(Box::new(code));

        let addr = app.instantiate_contract(code_id, sender, &Empty {}, &[], "verifier", None)
            .unwrap();

        app.wrap().query_wasm_smart::<()>(addr, &QueryMsg::VerifyProof {
            proof: serde_json::from_str(&fs::read_to_string("../server/proof.json").unwrap()).unwrap(),
            vk: serde_json::from_str(&fs::read_to_string("../server/vk.json").unwrap()).unwrap(),
        }).unwrap();
    }
}