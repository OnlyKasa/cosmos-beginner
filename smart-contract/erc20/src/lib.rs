use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Addr, StdError, Storage
};
use cosmwasm_storage::{singleton, singleton_read};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub balances: HashMap<String, u128>,
    pub allowances: HashMap<(String, String), u128>,
}

const STATE_KEY: &[u8] = b"state";

pub fn save_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton(storage, STATE_KEY).save(state)
}

pub fn load_state(storage: &dyn Storage) -> StdResult<State> {
    singleton_read(storage, STATE_KEY).load()
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let mut balances = HashMap::new();
    balances.insert(info.sender.to_string(), msg.initial_supply);

    let state = State {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: msg.initial_supply,
        balances,
        allowances: HashMap::new(),
    };

    save_state(deps.storage, &state)?;
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
    let mut state = load_state(deps.storage)?;
    let sender_balance = state.balances.get(&info.sender.to_string()).cloned().unwrap_or(0);

    if sender_balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    state.balances.insert(info.sender.to_string(), sender_balance - amount);
    let recipient_balance = state.balances.get(&recipient).cloned().unwrap_or(0);
    state.balances.insert(recipient, recipient_balance + amount);
    save_state(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "transfer"))
}

fn execute_approve(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    amount: u128,
) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    state.allowances.insert((info.sender.to_string(), spender.clone()), amount);
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "approve"))
}

fn execute_transfer_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: u128,
) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    let allowance = state.allowances.get(&(owner.clone(), info.sender.to_string())).cloned().unwrap_or(0);
    if allowance < amount {
        return Err(StdError::generic_err("Allowance exceeded"));
    }
    let owner_balance = state.balances.get(&owner).cloned().unwrap_or(0);
    if owner_balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }
    state.allowances.insert((owner.clone(), info.sender.to_string()), allowance - amount);
    state.balances.insert(owner.clone(), owner_balance - amount);
    let recipient_balance = state.balances.get(&recipient).cloned().unwrap_or(0);
    state.balances.insert(recipient, recipient_balance + amount);
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "transfer_from"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BalanceOf { owner } => query_balance_of(deps, owner),
        QueryMsg::TotalSupply {} => query_total_supply(deps),
    }
}

fn query_balance_of(deps: Deps, owner: String) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    let balance = state.balances.get(&owner).cloned().unwrap_or(0);
    to_binary(&balance)
}

fn query_total_supply(deps: Deps) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    to_binary(&state.total_supply)
}
