use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::verifier::types::{SP1ProofWithPublicValues, SP1VerifyingKey};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(())]
    VerifyProof {
        proof: SP1ProofWithPublicValues,
        vk: SP1VerifyingKey
    },
}
