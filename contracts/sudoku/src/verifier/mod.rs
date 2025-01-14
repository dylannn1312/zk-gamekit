use cosmwasm_std::HexBinary;
use crate::error::ContractError;
use crate::verifier::groth16::verify_groth16_proof;
use crate::verifier::types::{SP1ProofWithPublicValues,};
use crate::verifier::types::SP1Proof::Groth16;

pub mod types;
pub mod groth16;

pub fn verify_proof(proof: &SP1ProofWithPublicValues, vk: &HexBinary) -> Result<(), ContractError> {
    match &proof.proof {
        Groth16(proof) => verify_groth16_proof(proof, vk),
    }
}