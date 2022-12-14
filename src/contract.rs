#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, attr};
use cw2::{ set_contract_version};
//use serde::de::StdError;
// use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{PING_COUNT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ping-pong";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    PING_COUNT.save(deps.storage, &Uint64::zero())?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ping {} => ping(deps, env, info),
    }
}

fn ping(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let mut count: Uint64 = Uint64::zero();
    PING_COUNT.update(deps.storage, |mut my_value| -> Result<_, ContractError> {
        my_value = my_value + Uint64::from(1u8);
        // my_value = my_value.add(Uint64::from(1u8));
        count = my_value;
        Ok(my_value)
    })?;

    let resp = Response::new()
        .add_attribute("method", "ping")
        .add_attribute("ping_count", count.to_string())
        // .set_data(to_binary(&count).unwrap());
        .set_data(to_binary("pong").unwrap());
    Ok(resp)

    // let mut res = Response::new();
    // res.attributes.push(attr("ping_count", count));
    // res.data = Some (to_binary(&count)?);
    // Ok(res)*/
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount { .. } => {
            let count = PING_COUNT.load(deps.storage)?;
            to_binary(&GetCountResponse{ ping_count: count })
            // to_binary(&count)
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{ from_binary};

    #[test]
    fn test_ping() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator_address", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {}).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(res.attributes, vec![attr("action", "instantiate")]);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response : GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(count_response.ping_count, Uint64::zero());

        // Next code is used when on the query we return ++ to_binary(&count)
        // let count: Uint64 = from_binary(&res).unwrap();
        // assert_eq!(Uint64::zero(), count);

        let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Ping {}).unwrap();
        assert_eq!(res.attributes.len(),2);
        assert_eq!(res.attributes, vec![attr("method", "ping"), attr("ping_count", "1")]);

        let data: String = from_binary(&res.data.unwrap()).unwrap();
        assert_eq!(data,"pong");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response : GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(count_response.ping_count, Uint64::from(1u8));

        // Next code is used when on the query we return ++ to_binary(&count)
        // let count: Uint64 = from_binary(&res).unwrap();
        // assert_eq!(Uint64::from(1u8), count);
    }
}
