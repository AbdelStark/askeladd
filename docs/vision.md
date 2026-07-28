# The Vision: Verifiable Computation for the Nostr Revolution

> *"Cryptography shifts the balance of power from those with a monopoly on violence to those who comprehend mathematics and security design."* — Jacob Appelbaum

This document is the long-form version of why Askeladd exists. It expands on the original essay, [Powering Verifiable Computation for the Nostr Revolution](https://hackmd.io/@AbdelStark/nostr-dvm-verifiable-computation), and stands on the worldview laid out in [A Freedom Tech Manifesto](https://www.fgu.tech/).

## The market for computation has no integrity layer

Nostr gave the world a protocol for free speech: simple, resilient, impossible to shut down. [Data Vending Machines](https://nips.nostr.com/90) (NIP-90) extend it into something bigger — a protocol for *work*.

A DVM is an automated service provider living on the Nostr network. You broadcast a job request — translate this text, summarize this document, run this computation — attach a Lightning micropayment, and machines around the world compete to fulfill it. The result comes back as an ordinary Nostr event.

The properties that make this revolutionary:

- **Censorship-resistant.** There is no central operator to de-platform. As long as one relay carries your events, the market exists.
- **Permissionless.** Anyone can run a DVM offering anything. The barrier to selling computation to the entire planet is a Raspberry Pi and an internet connection.
- **Natively paid.** Lightning micropayments are part of the protocol flow, not a bolted-on checkout page. Pay for exactly what you use.
- **Globally competitive.** Providers compete on price, speed, and quality in the open. The race to the top benefits every user.

But a market where *anyone* can sell computation has an obvious flaw: **why should you believe the answer?**

Today, a DVM result is a claim. You paid a stranger's machine to compute something, and it sent you back bytes that *say* the computation was done correctly. For low-stakes jobs you can shrug and move on. For anything that matters — a financial calculation, a model inference you will act on, a result feeding another system — blind trust is not a foundation. It is an apology waiting to happen.

Every market in history solved this with intermediaries: escrow, regulators, reputation gatekeepers. Those solutions reintroduce exactly the gatekeepers Nostr exists to eliminate. There is only one solution that doesn't: **mathematics**.

## Don't trust. Verify.

A STARK proof is a cryptographic receipt that proves a computation was executed correctly. It has three properties that read like a wish list for a trustless computation market:

- **Succinct verification.** Checking the proof is vastly cheaper than redoing the computation. A phone can verify what a datacenter computed.
- **Zero knowledge.** The proof reveals nothing about private inputs. A provider can prove "I ran this model on your data correctly" without ever exposing the data or the model.
- **Transparency.** STARKs need no trusted setup — no ceremony, no secret parameters, no "trust us, we deleted the toxic waste." The only assumption is a hash function. They are also plausibly post-quantum secure.

Askeladd uses [Circle STARKs](https://eprint.iacr.org/2024/278) via StarkWare's [STWO](https://github.com/starkware-libs/stwo) prover — the fastest STARK prover in existence, designed for the small, efficient M31 field. Proof generation that used to be an academic curiosity now runs in seconds on commodity hardware, and even in a web browser.

With a proof attached, a DVM result stops being a claim and becomes a *fact*. The customer verifies locally. No escrow, no reputation system, no intermediary — just math. This is what it takes to build the next iteration of the internet: the **Integrity Web**.

## The agentic era makes this urgent

Here is the convergence this project bets on.

AI agents are becoming economic actors. They will browse, negotiate, buy, and sell on behalf of their owners — and they will overwhelmingly do it on open protocols, because agents cannot fill out KYC forms and do not have credit cards. Nostr + Lightning is the natural habitat: identity is a keypair, payment is a preimage, and the API is a websocket.

At the same time, the freedom tech movement faces a fork in the road laid out in [the manifesto](https://www.fgu.tech/): AI becomes either the ultimate instrument of centralized surveillance, or the great equalizer — **sovereign AI**, run locally, answering only to its owner; **open-source AI**, with public weights no entity controls; **unstoppable AI**, distributed beyond anyone's reach.

Sovereign AI has a computational reality problem. Your laptop cannot run every model. Your phone cannot prove every trace. Individuals will *rent* compute — the question is whether they rent it from a monopoly cloud that logs every query, or from an open market of machines they never have to trust.

That market only works with proofs:

- **Verifiable inference.** "Prove you ran model M on input X." The agent pays for the result *and* the proof, and verifies before acting. Compute becomes a commodity you can buy from anyone — which is exactly what makes open-source models unstoppable: no single provider can choke access, because every provider is interchangeable and every result is checkable.
- **Private delegation.** Zero-knowledge means the prover learns nothing it shouldn't. Your agent's queries, your data, your intentions stay yours.
- **Agent accountability.** Agents that hire other agents can demand proofs at every step. An economy of machines that never has to trust is an economy that cannot be captured.

Askeladd today proves toy programs — Fibonacci sequences, Poseidon hashes — because that is where you start when you want the *pattern* right. The pattern is what matters: **request → execute → prove → publish → verify**, all over Nostr, all permissionless, all checkable by anyone. Point it at a Cairo program, a WASM module, or eventually an ML trace, and the pattern does not change.

## What Askeladd demonstrates

Concretely, this repository shows:

1. **A verifiable DVM end to end.** A customer submits a proving job (kind `5600`), a prover agent executes it with STWO, publishes the result with the proof (kind `6600`), and the customer verifies it — see [architecture](architecture.md) and [protocol](protocol.md).
2. **Proofs in the browser.** The STWO prover and verifier compile to WebAssembly ([`crates/stwo_wasm`](../crates/stwo_wasm)). Verification needs no server, no trust in the marketplace frontend — the math checks out on your machine.
3. **A marketplace UI and a browser extension** ([`askeladd-dvm-marketplace`](../askeladd-dvm-marketplace/), [`thorfinn`](../thorfinn/)) showing what user-facing verifiable services look like.

It is a proof of concept and proud of it: small enough to read in an afternoon, complete enough to change how you think about trust online.

## The road ahead

- **Payments.** Wire NIP-57 zaps into the job lifecycle so proofs and payments settle together: no proof, no sats.
- **Pluggable programs.** Let anyone publish a provable program as a Nostr event (NIP-94/96) or on IPFS, and let provers fetch and execute it — an app store with no store owner.
- **Prover discovery and reputation.** NIP-89 announcements, so customers can find provers — and so good provers can build a name that is just a keypair.
- **Verifiable AI inference.** The endgame: provable model execution as a first-class job type, so sovereign agents can rent intelligence from the open market without trusting it.

## Why we build

Freedom tech has a formula, proven over decades: the printing press shattered the monopoly on truth, the internet shattered the monopoly on information, Bitcoin is shattering the monopoly on money, Nostr is shattering the monopoly on speech. AI will shatter the monopoly on expertise and cognitive scale — *if* we build it free.

Verifiable computation is the integrity layer of that free stack. It lets individuals rent the world's compute without kneeling to the world's gatekeepers. It turns "trust us" into "check it yourself" — the only sentence power has never learned to fake.

The revolution isn't just coming. It's here, it's provable, and it's unstoppable.

---

*Find the author on Nostr: `npub1hr6v96g0phtxwys4x0tm3khawuuykz6s28uzwtj5j0zc7lunu99snw2e29`*
