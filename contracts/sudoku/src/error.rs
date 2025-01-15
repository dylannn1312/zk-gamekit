use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    
    #[error("{0}")]
    PaymentErr(#[from] PaymentError),
    
    #[error("Invalid proof: {0}")]
    InvalidProof(String),

    #[error("Invalid solution: {0}")]
    InvalidSolution(String),
    
    #[error("Invalid action: {0}")]
    InvalidAction(String),
}
