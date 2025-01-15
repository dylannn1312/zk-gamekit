use cosmwasm_std::HexBinary;
use crate::error::ContractError;

pub fn verify_groth16_proof(proof: &HexBinary, vk: &HexBinary) -> Result<(), ContractError> {
    Ok(())
}