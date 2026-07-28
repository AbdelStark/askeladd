# Security Policy

## Status

Askeladd is a **proof of concept** and has not been audited. Do not rely on it to protect funds, secrets, or production workloads.

Known gaps, by design (see the [roadmap](README.md#roadmap)):

- No payment enforcement — jobs are processed without Lightning settlement.
- No sandboxing of uploaded programs — kind-5700 program launch is a stub and does not execute uploaded code.
- Demo keypairs ship in `.env.example` and `config/default.toml`. They are public; never use them for anything real.

## Reporting a vulnerability

If you find a security issue — cryptographic misuse, key leakage, verification bypass, or anything else — **please do not open a public issue**.

Contact the maintainer privately on Nostr:

```
npub1hr6v96g0phtxwys4x0tm3khawuuykz6s28uzwtj5j0zc7lunu99snw2e29
```

Include a description, affected versions/commit, and a reproduction if possible. You will get an acknowledgment as quickly as possible, credit in the fix (unless you prefer anonymity), and the thanks of everyone building on this stack.

## Cryptographic dependencies

The STARK machinery comes from [STWO](https://github.com/starkware-libs/stwo) (pinned by commit — see `Cargo.toml`). Issues in STWO itself should be reported upstream to StarkWare.
