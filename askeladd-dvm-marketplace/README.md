# Askeladd DVM Marketplace

A web marketplace for verifiable computation on Nostr. Publish proving jobs as [NIP-90](https://nips.nostr.com/90) Data Vending Machine requests, watch prover agents answer with STARK proofs, and **verify the proofs in your browser** — no server trust, ever.

Part of [Askeladd](../README.md): a censorship-resistant, globally verifiable proving network.

## What it does

- **Submit proving jobs** (Fibonacci, Wide Fibonacci, Poseidon) signed with your Nostr keys (NIP-07 extension or generated keys).
- **Track job results** as they land on the relays: output plus serialized STARK proof.
- **Verify proofs client-side** with the [STWO](https://github.com/starkware-libs/stwo) prover compiled to WebAssembly — verification happens on your machine, not ours.
- **Launch programs** (experimental): the kind-5700 flow for publishing programs to provers.

[Watch the demo](../docs/demo/askeladd-dvm-marketplace-demo.mp4)

## Prerequisites

The STWO WASM bindings live in `src/pkg/` — **a prebuilt copy is committed**, so the app works out of the box. To rebuild them from the Rust sources (after changing `crates/stwo_wasm`):

```bash
# From the repository root — requires wasm-pack (https://rustwasm.github.io/wasm-pack/)
cd crates/stwo_wasm
wasm-pack build --target web

# Copy the generated bindings into the app
rm -rf ../../askeladd-dvm-marketplace/src/pkg
cp -r pkg ../../askeladd-dvm-marketplace/src/
```

## Run it

```bash
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000). Point the app at any Nostr relays you like; jobs and results propagate by ordinary Nostr gossip.

## Stack

Next.js · TypeScript · Tailwind · [NDK](https://github.com/nostr-dev-kit/ndk) / nostr-tools · `stwo_wasm` (WASM)

## License

[MIT](../LICENSE)
