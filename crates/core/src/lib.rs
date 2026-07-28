//! # Askeladd
//!
//! A censorship-resistant, globally verifiable proving network.
//!
//! Askeladd connects [Nostr Data Vending Machines](https://nips.nostr.com/90)
//! (NIP-90) with STARK proofs: customers submit proving jobs as Nostr events,
//! prover agents answer with the result *and* a STARK proof of correct
//! execution, and anyone can verify — no trust required.
//!
//! ## Layout
//!
//! - [`dvm`] — the customer and service-provider agents, plus the wire types.
//! - [`prover_service`] — dispatches a job to the right program and proves it.
//! - [`verifier_service`] — verifies a job result's proof, locally.
//! - [`config`] — layered settings (TOML + environment).
//! - [`db`] — SQLite job ledger for idempotent proving.
//! - [`nostr_utils`], [`utils`] — tag parsing and input coercion.
//!
//! The STARK machinery itself lives in the companion `stwo_wasm` crate,
//! built on StarkWare's [STWO](https://github.com/starkware-libs/stwo) prover
//! (Circle STARKs).

pub mod config;
pub mod db;
pub mod dvm;
pub mod nostr_utils;
pub mod prover_service;
pub mod utils;
pub mod verifier_service;
