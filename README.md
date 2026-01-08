## Overview

We will build a customized PoR program based on your off-chain asset verification requirements. To get started, please complete this [Requirements Form](https://docs.google.com/forms/d/e/1FAIpQLSc9ijOzKQla4oOpSytvf4K3hjrfxAT-dGM0VUIFXAR94qn5Qw/viewform).

After we receive your requirements, our team will contact you to confirm several key details and further refine the program design. 

These details may include:

- **Exact data source URLs and asset API endpoints**, which may involve multiple APIs corresponding to different asset accounts
- **Asset disclosure scope**, defining the level of reserve detail that will be visible to your users
- **Verification frequency**, specifying how often the client program executes the verification process

The PoR program consists of two components: 

1. [Client](https://github.com/primus-labs/por-demo/tree/main/client)

   - A configurable client-side program deployed in your own environment
   - Responsible for authenticated data retrieval, data verification, and periodic reserve proof execution

2. [zkVM Program](https://github.com/primus-labs/por-demo/tree/main/zkvm-program)

   - A verifiable computation program that works alongside the client-side program and runs on the zkVM network
   - Aggregates asset data according to different disclosure scopes and generates zero-knowledge proofs for the computed results

   

## What we Deliver to You

Once the customized PoR program is built, we will deliver the following components:

1. **Client program**

   - Distributed via npm, with optional Docker-based deployment support

2. **Authentication parameters**, including:

   - **User Token**: Identifies your organization on the Primus side (one client corresponds to one user token)
   - **Project ID**: Used to associate each off-chain reserves program with its corresponding public explorer page

   

## What You Need to Configure

The client program is deployed **exclusively on your own server**, ensuring that you retain full control over all private credentials.

Before activating the program, you need to configure:

1. **Read-Only API Keys** for accessing your off-chain asset data

   - Multiple off-chain data sources are supported
   - Each data source can have multiple read-only API keys

2. **Authentication Parameters**

   - Configure the provided 'User Token' and 'Project ID' in the client program before running it

   

## How the PoR Program Works

Once the read-only API keys are configured, the client program is ready to run. After being started, it periodically completes off-chain reserve proofs according to the predefined execution cycle.

The client program execution flow is as follows:

1. Initiate a zkTLS attestation process to retrieve real-time asset balances from legitimate off-chain data source API endpoints.
2. Validate the off-chain data source and hashed asset details through Attestor nodes in the Primus Network, and propagate the validated data to the blockchain.
3. Submit the retrieved data to a verifiable computation backend (the zkVM network) via a TEE-assisted secure data processing channel.

The zkVM program then continues with the following steps:

4. Process asset data inside the zkVM according to the defined business logic, such as grouping assets by type or aggregating balances into a single reserve value.
5. Generate a zero-knowledge proof for the computed result.
6. Return the generated proofs and verified results to the client program.

Once these processes are complete, the verified off-chain reserve values are publicly disclosed on the explorer page.
