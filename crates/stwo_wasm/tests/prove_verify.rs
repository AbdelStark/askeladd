//! End-to-end prove/verify roundtrips for every built-in program.
//!
//! These run natively (not in WASM) but exercise exactly the code paths the
//! DVM prover agent and the browser verifier use.

use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_wasm::fibonacci::multi_fibonacci::MultiFibonacci;
use stwo_wasm::fibonacci::Fibonacci;
use stwo_wasm::poseidon::PoseidonStruct;
use stwo_wasm::wide_fibonacci::WideFibStruct;

const FIB_LOG_SIZE: u32 = 5;
const FIB_CLAIM: u32 = 443693538;

#[test]
fn fibonacci_proof_roundtrip() {
    let fib = Fibonacci::new(FIB_LOG_SIZE, BaseField::from(FIB_CLAIM));
    let proof = fib.prove().expect("proving should succeed");
    fib.verify(proof).expect("proof should verify");
}

#[test]
fn fibonacci_wrong_claim_fails() {
    // Proving a trace against a claim it does not satisfy must fail.
    let fib = Fibonacci::new(FIB_LOG_SIZE, BaseField::from(FIB_CLAIM + 1));
    assert!(fib.prove().is_err());
}

#[test]
fn fibonacci_proof_survives_serde_roundtrip() {
    // Proofs cross the wire as JSON inside Nostr events; a proof must still
    // verify after a serialization round-trip.
    let fib = Fibonacci::new(FIB_LOG_SIZE, BaseField::from(FIB_CLAIM));
    let proof = fib.prove().expect("proving should succeed");
    let json = serde_json::to_string(&proof).expect("proof serializes");
    let parsed = serde_json::from_str(&json).expect("proof deserializes");
    fib.verify(parsed).expect("proof should verify");
}

#[test]
fn multi_fibonacci_proof_roundtrip() {
    let multi = MultiFibonacci::new(
        vec![FIB_LOG_SIZE, FIB_LOG_SIZE],
        vec![BaseField::from(FIB_CLAIM), BaseField::from(FIB_CLAIM)],
    );
    let proof = multi.prove().expect("proving should succeed");
    multi.verify(proof).expect("proof should verify");
}

#[test]
fn wide_fibonacci_proof_roundtrip() {
    // 1024 sequences of 256 cells, packed into one proof.
    let wide = WideFibStruct::new(8, 10);
    let proof = wide
        .prove::<Blake2sMerkleHasher>()
        .expect("proving should succeed");
    wide.verify::<Blake2sMerkleHasher>(proof)
        .expect("proof should verify");
}

#[test]
fn poseidon_proof_roundtrip() {
    // 512 Poseidon permutations; log_n_rows = 9 - 3 = 6 >= MIN_LOG_N_ROWS.
    let poseidon = PoseidonStruct::new(9).expect("valid instance count");
    let proof = poseidon
        .prove::<Blake2sMerkleHasher>()
        .expect("proving should succeed");
    poseidon
        .verify::<Blake2sMerkleHasher>(proof)
        .expect("proof should verify");
}

#[test]
fn poseidon_rejects_undersized_traces() {
    assert!(PoseidonStruct::new(5).is_err());
}
