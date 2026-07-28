# The Askeladd Protocol

Askeladd speaks plain [NIP-90](https://nips.nostr.com/90) over Nostr. Every message is a standard, signed Nostr event; there is no side channel, no central server, no proprietary envelope. This document specifies the exact wire format so that anyone can interoperate — write your own customer, your own prover, your own verifier.

## Overview

| Kind  | Direction              | Purpose                                   |
| ----- | ---------------------- | ----------------------------------------- |
| 5600  | customer → network     | Proving job request (NIP-90 job request)  |
| 6600  | prover → network       | Proving job result (NIP-90 job result)    |
| 5700  | customer → network     | Program launch request (**experimental**) |

Kinds 5600 and 6600 live in the NIP-90 job-request / job-result ranges, so Askeladd jobs coexist with every other DVM on the network.

## 1. Job request — kind 5600

Published by the customer. Parameters travel in two complementary places:

- **NIP-90 `param` tags** — the interoperable, DVM-standard way. Each input is a `["param", "<key>", "<value>"]` tag; the requested output MIME type rides in `["output", "text/json"]`.
- **JSON content** — a self-describing payload that also selects *which* program to run.

### Content schema

```jsonc
{
  "request": { /* program inputs as a JSON object, e.g. { "log_size": 5, "claim": 443693538 } */ },
  "program": {
    "event_id": null,
    "unique_id": null,
    "pubkey_application": null,
    "inputs": { "log_size": "5", "claim": "443693538", "output": "text/json" },
    "inputs_types": null,
    "inputs_encrypted": null,
    "contract_reached": "InternalAskeladd",
    "contract_name": "FibonacciProvingRequest",
    "internal_contract_name": "FibonacciProvingRequest",
    "tags": null
  }
}
```

- `contract_reached` — how the prover obtains the program. `InternalAskeladd` (built-in programs) or `Ipfs` (not yet implemented).
- `internal_contract_name` — which built-in program to run. One of:
  - `FibonacciProvingRequest`
  - `WideFibonacciProvingRequest`
  - `MultiFibonacciProvingRequest`
  - `PoseidonProvingRequest`
  - `Custom("<name>")` — reserved for user-supplied programs.

### Program inputs

| Program           | Input              | Type | Meaning                                   |
| ----------------- | ------------------ | ---- | ----------------------------------------- |
| Fibonacci         | `log_size`         | u32  | log₂ of the trace length                  |
|                   | `claim`            | u32  | claimed Fibonacci value at that index     |
| Multi-Fibonacci   | `log_sizes`        | u32[] | one log₂ trace length per sequence       |
|                   | `claims`           | u32[] | one claim per sequence                    |
| Wide Fibonacci    | `log_fibonacci_size` | u32 | log₂ of each sequence's length (min 8)  |
|                   | `log_n_instances`  | u32  | log₂ of the number of sequences           |
| Poseidon          | `log_n_instances`  | u32  | log₂ of Poseidon permutations to prove (min 9) |

Size limits enforced by the prover agent (STWO AIR constraints):

- **Wide Fibonacci**: traces are packed `2^8` columns wide, so `log_fibonacci_size ≥ 8`, and `log_n_instances + log_fibonacci_size ≥ 12` (SIMD lanes need `2^4` rows minimum).
- **Poseidon**: instances are packed 8 per row, so `log_n_rows = log_n_instances − 3`; the AIR needs `log_n_rows ≥ 6`, i.e. `log_n_instances ≥ 9`.

## 2. Job result — kind 6600

Published by the prover agent, linked to the request with an `e` tag: `["e", "<job-request-event-id>", "", "reply"]`.

### Content schema

```jsonc
{
  "job_id": "<job-request-event-id-hex>",
  "response": {
    "response": { /* echo of the request inputs — the public statement */ },
    "proof": { /* serialized STARK proof (Blake2s Merkle channel) */ }
  }
}
```

The proof is the STWO `StarkProof<Blake2sMerkleHasher>` serialized with serde — commitments, sampled values, decommitments, query positions, and proof-of-work nonce. It appears **exactly once** in the event: proofs are large and relays enforce event-size limits. A real example lives at [`docs/demo/example_dvm_result_stark_proof.json`](demo/example_dvm_result_stark_proof.json).

### Proof sizes and relay limits

STARK proofs are big by nature. Measured sizes for the built-in programs (JSON, at demo parameters):

| Program | Parameters | Proof size |
| ------- | ---------- | ---------- |
| Fibonacci | `log_size=5` | ~8 KB |
| Multi-Fibonacci | two sequences, `log_size=5` | ~8 KB |
| Wide Fibonacci | `8, 8` (256 sequences × 256 cells) | ~47 KB |
| Poseidon | `log_n_instances=9` (512 permutations) | ~164 KB |

Most public relays cap events at 64–256 KB, so **Poseidon-class results do not propagate on many public relays**. Options, in order of practicality:

- Run your own relay with raised limits — the demo's [`config/nostr-rs-relay/config.toml`](../config/nostr-rs-relay/config.toml) allows 1 MB events.
- Use relays that advertise large `limitation.max_message_length` (NIP-11).
- Roadmap: distribute proofs out-of-band (NIP-94 / Blossom / IPFS) with only a commitment in the result event.

The prover agent waits for relay acknowledgment when publishing, so an event rejected for size is logged as a failed job rather than lost silently.

## 3. Verification

Verification needs **no network access and no trust in the prover**:

1. Deserialize the `proof` from the job result content.
2. Reconstruct the public statement from the request inputs (e.g. `log_size`, `claim`).
3. Run the STARK verifier (natively via `VerifierService`, or in the browser via the `stwo_wasm` WASM bindings).

The proof is self-contained and tied to a specific computation trace. Anyone who sees the result event — not just the customer — can verify it. That is the entire point: **receipts ask for trust; proofs remove the need for it.**

## 4. Program launch — kind 5700 (experimental)

Reserved for the pluggable-program roadmap: publish a program (WASM module, NIP-94 file metadata, or IPFS CID) and ask a prover to load it. The current service provider acknowledges launch requests but does not execute uploaded programs yet. Treat this kind as unstable.

## Notes for implementers

- All keys are standard Nostr keypairs; sign with your secret key, nothing else.
- Relays are the transport — pick any set you like; jobs and results propagate by ordinary Nostr gossip.
- Provers should idempotently skip job IDs they have already completed (the reference agent tracks state in a local SQLite ledger).
- Nothing in the protocol binds payment yet; Lightning settlement (NIP-57 zaps) is on the [roadmap](../README.md#roadmap).
