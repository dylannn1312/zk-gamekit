use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr};
use crate::verifier::types::SP1ProofWithPublicValues;

#[cw_serde]
pub(crate) struct SudokuGame {
    pub initial_state: Vec<(u8, u8)>,
    pub creator: Addr,
    pub deposit_price: u128,
    pub denom: String,
    pub players: Vec<Addr>,
    pub solution: Option<GameSolution>,
    pub winner: Option<Addr>,
    pub claimed: bool
}

#[cw_serde]
pub enum GameSolution {
    Public(Vec<u8>),
    Private(SP1ProofWithPublicValues)
}