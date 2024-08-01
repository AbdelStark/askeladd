export const KIND_JOB_RESULT = 6600
export const KIND_JOB_REQUEST = 5600


export interface JobResultProver {
    job_id: string;
    response: {
        // params in a JSON object
        // proof
        proof: any
        // proof: StarkProof
    }
}

export interface StarkProof {
    commitments: any[]; //  Uint8Array 32
    lookup_values: Map<string, string>;
    commitment_scheme_proof: CommitmentSchemeProof
}

interface DecommitmentProof {
    column_witness: any[], hash_witness: any[]
}

interface ProofInnerLayer {
    evals_subset: any[],
    decommitment: any[],
    decomposition_coef: any[],
    commitment: any[]
}
/** @TODO finish to implement correctly */
export interface CommitmentSchemeProof {
    sampled_values: any[];
    commitments: any[]; // Uint8Array 32
    queried_values: any[];
    proof_of_work: {
        nonce: string
    };
    decommitments: DecommitmentProof[];
    fri_proof: {
        inner_layers: ProofInnerLayer[] | any[];
        last_layer_poly: {
            coeffs: any[];
        }
    };
}

export enum ASKELADD_KINDS {
    KIND_JOB_REQUEST = 5600,
    KIND_JOB_RESULT = 6600,
    // KIND_SUBMIT_PROGRAM
}

export enum ASKELADD_KINDS_NAME {
    KIND_JOB_REQUEST = "Job request",
    KIND_JOB_RESULT = "Job result",
    KIND_SUBMIT_PROGRAM = "Submit result",
}
// export const ASKELADD_KINDS= {
//     KIND_JOB_REQUEST,
//     KIND_JOB_RESULT
// }

export interface IFormRecommendedApplicationHandlerEvent {

}

export enum ConfigHandle {
    SPECIFIC_KIND,
    ALL_KIND
}