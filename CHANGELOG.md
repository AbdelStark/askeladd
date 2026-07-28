# Changelog

All notable changes to Askeladd are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Fixed

- **Build reproducibility.** `stwo-prover` is pinned to a commit (`14e41f9`, 2024-08-07) instead of tracking the moving `stwo` default branch, and `Cargo.lock` is curated and committed. The workspace builds again from a clean checkout with the pinned toolchain.
- **Fibonacci proving worked at all.** The prover committed no trace; it now uses `commit_and_prove` correctly, with prove/verify roundtrip tests.
- **Multi-Fibonacci implemented.** It was a stub that always failed; it now proves and verifies for real.
- **Poseidon double subtraction.** `prove()` treated `log_n_rows` as `log_n_instances`, shrinking the trace and failing every job. The Fiat–Shamir transcript is also replayed consistently between prover and verifier.
- **Wide Fibonacci moved to the SIMD AIR**, the only upstream-tested variant; the CPU wrapper failed for every input.
- **Verification actually verifies.** The old verifier panicked on any job result (missing serde tag) and returned `Ok(())` without checking Poseidon/Wide-Fibonacci proofs. All programs are now verified against their echoed public inputs.
- **Database schema setup.** Both `CREATE TABLE` statements were packed into one `execute` call, so the second table was never created; schema setup now uses `execute_batch`.
- **`config/default.toml` carried a `prover_agent_pk` that does not match `prover_agent_sk`** — customers filtered results by the wrong author and could never see their job results.
- **Result events carried the proof twice**, doubling their size and pushing them over relay event-size limits; the proof now travels exactly once.
- **Publish raced the relay handshake** and oversized results vanished silently: both agents now wait for a live relay connection and for relay acknowledgment of published events.
- **Demo scripts** referenced a non-existent `user-cli` compose service and ran compose from the wrong directory.

### Changed

- **Documentation overhaul.** New README anchored in the project thesis (*Powering Verifiable Computation for the Nostr Revolution*) and the freedom tech worldview; new `docs/vision.md`, `docs/architecture.md`, and `docs/protocol.md` (superseding `docs/proposal.md`).
- **`dvm_customer` is now a real CLI** (`clap`): `fibonacci`, `poseidon`, `wide-fibonacci`, `multi-fibonacci`, and `demo` subcommands with sane defaults and timeouts.
- **Library code no longer panics on the unhappy path.** Typed errors (`thiserror`) throughout `askeladd-core`; `unwrap`/`expect` removed from request handling; `println!` debug noise replaced with structured logging.
- **Docker image** builds with the pinned nightly toolchain on a current Alpine base.
- **End-to-end validated on public relays**: Fibonacci, Multi-Fibonacci, and Wide Fibonacci jobs prove and verify over the live network; Poseidon works end-to-end on relays with raised event-size limits (the bundled relay config allows 1 MB events).
- **WASM bindings rebuilt and verified** — `wasm-pack build --target web` with the pinned toolchain, smoke-tested in Node (prove/verify roundtrip, wrong claims rejected). The marketplace ships the fresh `src/pkg`, and passes `tsc --noEmit` and `next build`.
- Added `CONTRIBUTING.md`, `SECURITY.md`, and this changelog.

### Removed

- Stale root-level `tests/e2e_test.rs` (referenced long-gone types) and the broken e2e CI workflow that ran it.
- Duplicate `nostr-sdk` build in the CLI crate, plus a handful of unused dependencies.

## [0.1.1] - 2024-08-07

Last pre-overhaul state: NIP-90 proving flow over Nostr with STWO, SQLite job ledger, WASM bindings, Next.js marketplace, Thorfinn browser extension.
