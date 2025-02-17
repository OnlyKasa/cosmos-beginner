use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Addr, StdError
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Transfer { recipient: String, amount: u128 },
    Approve { spender: String, amount: u128 },
    TransferFrom { owner: String, recipient: String, amount: u128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    BalanceOf { owner: String },
    TotalSupply {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub balances: std::collections::HashMap<String, u128>,
    pub allowances: std::collections::HashMap<(String, String), u128>,
}

const CONTRACT_NAME: &str = "erc20-token";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: msg.initial_supply,
        balances: std::collections::HashMap::new(),
        allowances: std::collections::HashMap::new(),
    };

    let mut balances = state.clone().balances;
    balances.insert(info.sender.to_string(), msg.initial_supply);

    deps.storage.set(b"state", &serde_json::to_vec(&state).map_err(|e| StdError::generic_err(e.to_string()))?);

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => execute_transfer(deps, info, recipient, amount),
        ExecuteMsg::Approve { spender, amount } => execute_approve(deps, info, spender, amount),
        ExecuteMsg::TransferFrom { owner, recipient, amount } => execute_transfer_from(deps, info, owner, recipient, amount),
    }
}

fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: u128,
) -> StdResult<Response> {
    let serialized = deps.storage.get(b"state").ok_or(StdError::generic_err("State not found"))?;
    let mut state: State = serde_json::from_slice(&serialized).map_err(|e| {
        StdError::generic_err(format!("Failed to decode state: {}", e))
    })?;

    let sender_balance = state.balances.get(&info.sender.to_string()).unwrap_or(&0);

    if *sender_balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    state.balances.insert(info.sender.to_string(), sender_balance - amount);
    let recipient_balance = state.balances.get(&recipient).unwrap_or(&0);
    state.balances.insert(recipient, recipient_balance + amount);

    let tmp = state.clone();
    deps.storage.set(b"state", &serde_json::to_vec(&tmp).map_err(|e| StdError::generic_err(e.to_string()))?);

    Ok(Response::new().add_attribute("method", "transfer"))
}

fn execute_approve(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    amount: u128,
) -> StdResult<Response> {
    let serialized = deps.storage.get(b"state").ok_or(StdError::generic_err("State not found"))?;
    let mut state: State = serde_json::from_slice(&serialized).map_err(|e| {
        StdError::generic_err(format!("Failed to decode state: {}", e))
    })?;

    state.allowances.insert((info.sender.to_string(), spender), amount);

    deps.storage.set(b"state", &serde_json::to_vec(&state).map_err(|e| StdError::generic_err(e.to_string()))?);

    Ok(Response::new().add_attribute("method", "approve"))
}


fn execute_transfer_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: u128,
) -> StdResult<Response> {

    let serialized = deps.storage.get(b"state").ok_or(StdError::generic_err("State not found"))?;
    let mut state: State = serde_json::from_slice(&serialized).map_err(|e| {
        StdError::generic_err(format!("Failed to decode state: {}", e))
    })?;

    let owner_balance = state.balances.get(&owner).unwrap_or(&0);

    if *owner_balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    let allowance = state.allowances.get(&(owner.clone(), info.sender.to_string())).unwrap_or(&0);

    if *allowance < amount {
        return Err(StdError::generic_err("Allowance exceeded"));
    }

    state.balances.insert(owner.clone(), owner_balance - amount);
    let recipient_balance = state.balances.get(&recipient).unwrap_or(&0);
    state.balances.insert(recipient, recipient_balance + amount);

    let new_allowance = allowance - amount;
    state.allowances.insert((owner.clone(), info.sender.to_string()), new_allowance);

    deps.storage.set(b"state", &serde_json::to_vec(&state).map_err(|e| StdError::generic_err(e.to_string()))?);


    Ok(Response::new().add_attribute("method", "transfer_from"))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BalanceOf { owner } => query_balance_of(deps, owner),
        QueryMsg::TotalSupply {} => query_total_supply(deps),
    }
}

fn query_balance_of(deps: Deps, owner: String) -> StdResult<Binary> {
    let serialized = deps.storage.get(b"state").ok_or(StdError::generic_err("State not found"))?;
    let state: State = serde_json::from_slice(&serialized).map_err(|e| {
        StdError::generic_err(format!("Failed to decode state: {}", e))
    })?;
    let balance = state.balances.get(&owner).unwrap_or(&0);
    to_binary(&balance)
}


fn query_total_supply(deps: Deps) -> StdResult<Binary> {
    let serialized = deps.storage.get(b"state").ok_or(StdError::generic_err("State not found"))?;
    let state: State = serde_json::from_slice(&serialized).map_err(|e| {
        StdError::generic_err(format!("Failed to decode state: {}", e))
    })?;
    to_binary(&state.total_supply)
}