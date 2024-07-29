export interface JobResultProver {
    job_id:string;
    response: {
        // params in a JSON object
         
        // proof
        proof:StarkProof
    }
}

export interface StarkProof {
    commitments:string[];
    lookup_values:Map<string, string>;
    commitment_scheme_proof:CommitmentSchemeProof
}

/** @TODO finish to implement correctly */
export interface CommitmentSchemeProof {
    sampled_values:string[];
    decommitments:string[];
    queried_values:string[];
    proof_of_work:{
        nonce:string
    };
    fri_proof:{
        inner_layers:{evals_subset:number[][][], }[]
        decommitment:{
             hash_witness:number[][]
        }
    };
}
