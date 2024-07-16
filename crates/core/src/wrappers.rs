use std::mem;

use serde::{Deserialize, Serialize};
use stwo_prover::core::prover::StarkProof;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarkProofWrapper(Vec<u8>);

/// Warning: This is a hack to get around the fact that StarkProof is not serializable.
/// TODO: Remove this when StarkProof is serializable.
impl From<StarkProof> for StarkProofWrapper {
    fn from(proof: StarkProof) -> Self {
        let bytes = unsafe {
            let ptr = &proof as *const StarkProof as *const u8;
            std::slice::from_raw_parts(ptr, mem::size_of::<StarkProof>())
        };
        StarkProofWrapper(bytes.to_vec())
    }
}

/// Warning: This is a hack to get around the fact that StarkProof is not serializable.
/// TODO: Remove this when StarkProof is serializable.
impl TryFrom<StarkProofWrapper> for StarkProof {
    type Error = &'static str;

    fn try_from(wrapper: StarkProofWrapper) -> Result<Self, Self::Error> {
        if wrapper.0.len() != mem::size_of::<StarkProof>() {
            return Err("Invalid byte length for StarkProof");
        }

        let proof = unsafe {
            let ptr = wrapper.0.as_ptr() as *const StarkProof;
            ptr.read()
        };

        Ok(proof)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid lookup value")]
    InvalidLookupValue,
    #[error("Invalid sampled value")]
    InvalidSampledValue,
    #[error("Invalid Merkle proof")]
    InvalidMerkleProof,
}
