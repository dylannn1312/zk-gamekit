use crate::error::ContractError;
use crate::verifier::types::Groth16Bn254Proof;

pub fn verify_groth16_proof(proof: &Groth16Bn254Proof) -> Result<(), ContractError> {
    todo!()
}