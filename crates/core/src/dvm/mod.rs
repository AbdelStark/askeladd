//! Data Vending Machine (NIP-90) building blocks.
//!
//! The Askeladd protocol speaks plain NIP-90 over Nostr: customers publish
//! job requests (kind 5600), provers publish job results carrying STARK
//! proofs (kind 6600). See `docs/protocol.md` for the full wire format.

pub mod customer;
pub mod service_provider;

/// Protocol-wide constants.
pub mod constants {

    /// Human-readable name of this DVM.
    pub const DVM_NAME: &str = "askeladd";
    /// One-line description used in announcements and banners.
    pub const DVM_DESCRIPTION: &str = "Censorship-resistant global proving network.";
    /// Service identifier for the proving service.
    pub const SERVICE_NAME: &str = "generate-zk-proof";
    /// Askeladd protocol version.
    pub const VERSION: &str = "0.1.0";
    /// NIP-90 job request kind used for proving requests.
    pub const JOB_REQUEST_KIND: u16 = 5600;
    /// NIP-90 job result kind used for proving results.
    pub const JOB_RESULT_KIND: u16 = 6600;
    /// Experimental kind for publishing programs to provers (not yet implemented).
    pub const JOB_LAUNCH_PROGRAM_KIND: u16 = 5700;
}

/// Wire types exchanged between customers and provers.
pub mod types {
    use std::collections::HashMap;

    use nostr_sdk::{EventId, Tag};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use stwo_prover::core::prover::StarkProof;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

    /// A proving job request, carried as the JSON content of a kind-5600 event.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GenerateZKPJobRequest {
        /// Program inputs as a JSON object (e.g. `{"log_size": 5, "claim": 443693538}`).
        pub request: Value,
        /// Which program to run and how to reach it. Absent means "no program",
        /// which provers reject.
        pub program: Option<ProgramParams>,
    }

    impl GenerateZKPJobRequest {
        pub fn new(request: Value, program: Option<ProgramParams>) -> Self {
            Self { request, program }
        }
    }

    /// How the prover obtains the program to execute.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum ContractUploadType {
        /// One of the programs built into the prover agent.
        InternalAskeladd,
        /// WASM program fetched from IPFS (not yet implemented).
        Ipfs,
    }

    /// The programs built into the reference prover agent.
    ///
    /// The serialized names are part of the wire protocol — do not rename
    /// variants without a migration. (The `Fibonacci` variant accepts the
    /// historical misspelling for backward compatibility.)
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ProgramInternalContractName {
        #[serde(alias = "FibonnacciProvingRequest")]
        FibonacciProvingRequest,
        PoseidonProvingRequest,
        WideFibonacciProvingRequest,
        MultiFibonacciProvingRequest,
        Custom(String),
    }

    /// Program selection and inputs for a proving job.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProgramParams {
        /// Event carrying the program, for externally published programs.
        pub event_id: Option<EventId>,
        /// Caller-chosen deduplication ID.
        pub unique_id: Option<String>,
        /// Restrict the job to a specific application pubkey (one-to-one marketplace).
        pub pubkey_application: Option<String>,
        /// String inputs, also mirrored as NIP-90 `param` tags on the request event.
        pub inputs: Option<HashMap<String, String>>,
        /// Optional type annotations for `inputs`.
        pub inputs_types: Option<HashMap<String, String>>,
        /// Reserved for encrypted inputs (not yet implemented).
        pub inputs_encrypted: Option<HashMap<String, String>>,
        /// How the prover obtains the program.
        pub contract_reached: ContractUploadType,
        /// Program name (for externally published programs).
        pub contract_name: Option<String>,
        /// Which built-in program to run.
        pub internal_contract_name: Option<ProgramInternalContractName>,
        /// Extra Nostr tags to attach to the request event.
        pub tags: Option<Vec<Tag>>,
        // TODO: payment terms (minimum sats) once Lightning settlement lands.
    }

    /// A proving job result, carried as the JSON content of a kind-6600 event.
    ///
    /// `response` holds the serialized [`GenericProvingResponse`] — the echoed
    /// inputs plus the STARK proof. The proof appears exactly once on the
    /// wire: proofs are large (kilobytes to hundreds of kilobytes), and every
    /// byte counts against relay event-size limits.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GenerateZKPJobResult {
        /// Event ID of the job request this answers.
        pub job_id: String,
        /// The serialized [`GenericProvingResponse`]: `{ response, proof }`.
        pub response: Value,
    }

    impl GenerateZKPJobResult {
        pub fn new(job_id: String, response: Value) -> Self {
            Self { job_id, response }
        }
    }

    /// What a prover returns for any program: the echoed inputs and the proof.
    ///
    /// The echoed inputs double as the public statement a verifier needs to
    /// check the proof against.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GenericProvingResponse {
        pub response: Value,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }

    impl GenericProvingResponse {
        pub fn new(response: Value, proof: StarkProof<Blake2sMerkleHasher>) -> Self {
            Self { proof, response }
        }
    }

    /// Inputs for the Fibonacci program: prove knowledge that the Fibonacci-squared
    /// sequence of length `2^log_size` ends at `claim`.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FibonacciProvingRequest {
        pub log_size: u32,
        pub claim: u32,
    }

    /// Inputs for the multi-Fibonacci program: several sequences in one proof.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MultiFibonacciProvingRequest {
        pub log_sizes: Vec<u32>,
        pub claims: Vec<u32>,
    }

    /// Inputs for the wide-Fibonacci program: `2^log_n_instances` sequences of
    /// length `2^log_fibonacci_size` in a single proof.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct WideFibonacciProvingRequest {
        pub log_fibonacci_size: u32,
        pub log_n_instances: u32,
    }

    /// Inputs for the Poseidon program: `2^log_n_instances` Poseidon permutations.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PoseidonProvingRequest {
        pub log_n_instances: u32,
    }
}
