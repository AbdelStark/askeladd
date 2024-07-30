<div align="center">
    <img src="docs/img/askeladd-text.png" alt="Askeladd" height=256>
    <h2>Censorship-resistant global proving network.</h2>

<a href="https://github.com/AbdelStark/askeladd/actions/workflows/ci.yaml"><img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/AbdelStark/askeladd/ci.yaml?style=for-the-badge" height=30></a>

<!--a href="https://github.com/AbdelStark/askeladd/actions/workflows/e2e-test.yaml"><img alt="GitHub Workflow Status E2E Tests" src="https://img.shields.io/github/actions/workflow/status/AbdelStark/askeladd/e2e-test.yaml?style=for-the-badge" height=30></a-->

<a href="https://bitcoin.org/"> <img alt="Bitcoin" src="https://img.shields.io/badge/Bitcoin-000?style=for-the-badge&logo=bitcoin&logoColor=white" height=30></a>
<a href="https://nostr.com/"> <img alt="Nostr" src="https://img.shields.io/badge/Nostr-000?style=for-the-badge" height=30></a>
<a href="https://lightning.network/"><img src="https://img.shields.io/badge/Ligthning Network-000.svg?&style=for-the-badge&logo=data:image/svg%2bxml;base64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI%2FPg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIEZpcmV3b3JrcyAxMCwgRXhwb3J0IFNWRyBFeHRlbnNpb24gYnkgQWFyb24gQmVhbGwgKGh0dHA6Ly9maXJld29ya3MuYWJlYWxsLmNvbSkgLiBWZXJzaW9uOiAwLjYuMSAgLS0%2BDQo8IURPQ1RZUEUgc3ZnIFBVQkxJQyAiLS8vVzNDLy9EVEQgU1ZHIDEuMS8vRU4iICJodHRwOi8vd3d3LnczLm9yZy9HcmFwaGljcy9TVkcvMS4xL0RURC9zdmcxMS5kdGQiPg0KPHN2ZyBpZD0iYml0Y29pbl9saWdodG5pbmdfaWNvbi5mdy1QYWdlJTIwMSIgdmlld0JveD0iMCAwIDI4MCAyODAiIHN0eWxlPSJiYWNrZ3JvdW5kLWNvbG9yOiNmZmZmZmYwMCIgdmVyc2lvbj0iMS4xIg0KCXhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHhtbDpzcGFjZT0icHJlc2VydmUiDQoJeD0iMHB4IiB5PSIwcHgiIHdpZHRoPSIyODBweCIgaGVpZ2h0PSIyODBweCINCj4NCgk8cGF0aCBpZD0iRWxsaXBzZSIgZD0iTSA3IDE0MC41IEMgNyA2Ni43NjkgNjYuNzY5IDcgMTQwLjUgNyBDIDIxNC4yMzEgNyAyNzQgNjYuNzY5IDI3NCAxNDAuNSBDIDI3NCAyMTQuMjMxIDIxNC4yMzEgMjc0IDE0MC41IDI3NCBDIDY2Ljc2OSAyNzQgNyAyMTQuMjMxIDcgMTQwLjUgWiIgZmlsbD0iI2Y3OTMxYSIvPg0KCTxwYXRoIGQ9Ik0gMTYxLjE5NDMgNTEuNSBDIDE1My4yMzQ5IDcyLjE2MDcgMTQ1LjI3NTYgOTQuNDEwNyAxMzUuNzI0NCAxMTYuNjYwNyBDIDEzNS43MjQ0IDExNi42NjA3IDEzNS43MjQ0IDExOS44MzkzIDEzOC45MDgxIDExOS44MzkzIEwgMjA0LjE3NDcgMTE5LjgzOTMgQyAyMDQuMTc0NyAxMTkuODM5MyAyMDQuMTc0NyAxMjEuNDI4NiAyMDUuNzY2NyAxMjMuMDE3OSBMIDExMC4yNTQ1IDIyOS41IEMgMTA4LjY2MjYgMjI3LjkxMDcgMTA4LjY2MjYgMjI2LjMyMTQgMTA4LjY2MjYgMjI0LjczMjEgTCAxNDIuMDkxOSAxNTMuMjE0MyBMIDE0Mi4wOTE5IDE0Ni44NTcxIEwgNzUuMjMzMyAxNDYuODU3MSBMIDc1LjIzMzMgMTQwLjUgTCAxNTYuNDE4NyA1MS41IEwgMTYxLjE5NDMgNTEuNSBaIiBmaWxsPSIjZmZmZmZmIi8%2BDQo8L3N2Zz4%3D" alt="Bitcoin Lightning" height="30"></a>
<a href="https://www.rust-lang.org/"> <img alt="Rust" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" height=30></a>

</div>

## About

Askeladd is a censorship-resistant global proving network, for anyone to be able to generate validity proofs, using [STWO](https://github.com/starkware-libs/stwo) prover, and verify them. It enables to submit proving request to the network and retrieve the generated proof for any given request.
Askeladd leverages [Nostr](https://github.com/nostr-protocol/nostr) for the communication layer, to gossip the proving requests and generated proofs, following the [NIP-90 - Data Vending Machine](https://nips.nostr.com/90) specification.

> **Disclaimer:** Askeladd is only a proof of concept and should not be used in a production environment. It's a work in progress as a showcase of the STWO prover and the Nostr protocol.

Check out this video demonstration of Askeladd in action:

[![asciicast](https://asciinema.org/a/670103.png)](https://asciinema.org/a/670103)

## Architecture

```mermaid
graph LR
    Customer((DVM Customer))
    SP[DVM Service Provider]
    Nostr[Nostr Network]
    STWO_P[STWO Prover]
    STWO_V[STWO Verifier]

    Customer -->|1. Submit Proving Request<br>Kind: 5600| Nostr
    Nostr -->|2. Fetch Request| SP
    SP -->|3. Generate Proof| STWO_P
    SP -->|4. Publish Proof<br>Kind: 6600| Nostr
    Nostr -->|5. Fetch Proof| Customer
    Customer -->|6. Verify Proof| STWO_V

    classDef customer fill:#f9d71c,stroke:#333,stroke-width:2px;
    classDef provider fill:#66b3ff,stroke:#333,stroke-width:2px;
    classDef network fill:#333,stroke:#333,stroke-width:2px,color:#fff;
    classDef prover fill:#ff9999,stroke:#333,stroke-width:2px;
    classDef verifier fill:#b19cd9,stroke:#333,stroke-width:2px;

    class Customer customer;
    class SP provider;
    class Nostr network;
    class STWO_P prover;
    class STWO_V verifier;
```

Typical flow:

1. User submits a proving request to the network (DVM Job Kind `5600`)
2. A DVM Service Provider picks the request and generates a proof using the STWO prover.
3. The proof is published to the Nostr network (DVM Job Result Kind `6600`).
4. The DVM Customer can verify the proof using the STWO verifier.

## Running the demo

### Using docker-compose

```bash
./scripts/demo_docker_compose_local_relayer.sh
```

### Manually

Create a `.env` file, you can use the `.env.example` file as a reference.

```bash
cp .env.example .env

# Set the relay URL in the .env file

# Using a local relay
APP_SUBSCRIBED_RELAYS=ws://localhost:8080

# Or a public relay
APP_SUBSCRIBED_RELAYS=wss://relay.nostr.net
```

(Optional, if you want to run a local nostr relay) In a terminal, run the local nostr relay:

```bash
docker run -p 8080:8080 scsibug/nostr-rs-relay
```

In a terminal, run the prover agent:

```bash
cargo run --bin dvm_service_provider
```

In a terminal, run the user CLI:

```bash
cargo run --bin dvm_customer
```

The user CLI binary will submit a proving request to the Nostr network. The prover agent will generate a proof for the request and publish it to the Nostr network. The user CLI binary will be able to verify the proof.

## 🤝 Contributing

We love contributions! If you have ideas for improvements or find any issues, please open an issue or submit a pull request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

This demo is powered by the amazing [STWO Prover](https://github.com/starkware-libs/stwo) from StarkWare. A big thank you to the StarkWare team and all contributors!

## 📚 Resources

- [Nostr Rust relay](https://github.com/scsibug/nostr-rs-relay/)
- [Nostr web tooling](https://nostrtool.com/)
- [NIP-90 - Data Vending Machine](https://nips.nostr.com/90)
- [Vendata.io](https://vendata.io/dvms)
- [Data Vending Machines](https://www.data-vending-machines.org/)
