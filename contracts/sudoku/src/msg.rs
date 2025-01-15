use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary};
use crate::game::GameSolution;

#[cw_serde]
pub struct InstantiateMsg {
    pub vk: HexBinary
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(())]
    QueryRoom {
        room_id: u64,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateNewRoom {
        initial_state: Vec<(u8, u8)>,
        deposit_price: u128,
        denom: String
    },
    JoinRoom {
        room_id: u64,
    },
    SubmitSolution {
        room_id: u64,
        solution: GameSolution,
    },
    ClaimReward {
        room_id: u64,
    }
}
