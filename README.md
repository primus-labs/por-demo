## Technical Overview

Primus enables off-chain reserve verification with end-to-end cryptographic guarantees by combining zkTLS, TEE, and zkVM-based verifiable computation on a decentralized proving network.

1. **zkTLS for Authenticated Data Retrieval**

Institutions deploy a custom PoR program in their own environment, where zkTLS is used to retrieve real-time asset balances from off-chain sources.

zkTLS produces a proof that:

- the data is fetched from the legitimate API endpoint
- over a trusted TLS session
- without exposing API credentials or raw account details

This off-chain data source, along with the hashed asset details, is validated by Attestor nodes in the Primus Network and then propagated to the blockchain.

2. **TEE-Assisted Secure Data Processing**

The retrieved off-chain data is additionally processed inside a Trusted Execution Environment (TEE), ensuring:

- data confidentiality throughout computation
- secure communication to the decentralized proving network

The TEE establishes a secure channel to the verifiable computation backend.

3. **zkVM for Verifiable Aggregation of Reserves**

Through the TEE, the committed data is sent to a zkVM network (e.g., powered by Succinct or other partners), where the PoR program performs:

- grouping by token/asset type
- aggregation of balances into a unified reserve value, which could be disclosed to the public

A zero-knowledge proof is then generated to confirm that the publicly disclosed reserve value is correctly computed from authentic, privately held balances — without revealing any sensitive account-level data.

## How We Achieve This

Primus separates the PoR process across two cooperating components:

* **[Client](./client/README.md)**: A configurable client-side program deployed in the institution’s environment, responsible for authenticated data retrieval, data verification, and periodic reserve proof execution based on off-chain asset verification requirements.

* **[zkVM Program](./zkvm-program/README.md)**: A verifiable computation program that aggregates asset data according to different disclosure scopes and generates zero-knowledge proofs for the computed results.
