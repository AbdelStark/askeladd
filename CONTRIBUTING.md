# Contributing to Askeladd

First: thank you. Askeladd is freedom tech — it exists so that anyone, anywhere, can verify computation without asking permission. Every contribution pushes that forward.

## Ways to contribute

- **Code.** Bug fixes, new provable programs, protocol improvements, payment integration.
- **Documentation.** Typos, clarifications, translations, tutorials. If you were confused by something, fixing it for the next person is a real contribution.
- **Ideas.** Open an issue. Half of this project is a thesis; arguing about it is how it gets better.

## Development setup

```bash
git clone https://github.com/AbdelStark/askeladd.git
cd askeladd

# The repo pins its Rust toolchain (rust-toolchain.toml); rustup picks it up automatically.
cargo check

# Run the test suite
cargo test

# Format and lint — CI enforces both
./scripts/rust_fmt.sh
./scripts/clippy.sh
```

For an end-to-end run you need a Nostr relay (`docker run -p 8080:8080 scsibug/nostr-rs-relay`) and the two binaries — see the [Quickstart](README.md#quickstart).

## Ground rules

- **Small, reviewable PRs.** One idea per pull request. Refactors separate from behavior changes.
- **Green CI or it doesn't merge.** `rustfmt`, `clippy` (warnings denied), and tests must pass.
- **No panics in library code.** `crates/core` returns typed errors; unwraps belong in tests and demo binaries only.
- **Docs are code.** If you change behavior, update `docs/` and the README in the same PR.
- **Freedom tech values.** Permissionless, open source, resilient, privacy-first, built from first principles. If a change weakens any of those, it needs a very good reason.

## Commit style

Short imperative subject lines, conventional prefixes welcome (`feat:`, `fix:`, `docs:`, `refactor:`). Explain the *why* in the body when it isn't obvious.

## License

By contributing you agree your work is released under the [MIT License](LICENSE).
