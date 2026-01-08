## Overview

We will build a customized PoR client program based on your off-chain asset verification requirements. To get started, please complete this [Requirements Form](https://docs.google.com/forms/d/e/1FAIpQLSc9ijOzKQla4oOpSytvf4K3hjrfxAT-dGM0VUIFXAR94qn5Qw/viewform).

After we receive your requirements, our team will contact you to confirm several key details and further refine the program design. 

Details may include:

- **Exact data source URLs and asset API endpoints**, which may involve multiple APIs corresponding to different asset accounts
- **Asset disclosure scope**, defining the level of reserve detail that will be visible to your users
- **Verification frequency**, specifying how often the client program executes the verification process

## What we Deliver

Once the customized program is built, we will deliver the following components:

1. **Client-side program**
   - Distributed via the npm repository
   - Docker-based deployment is also supported if required
2. **Authentication parameters**, including:
   - **User Token**: Identifies your organization on the Primus side (one client corresponds to one user token)
   - **Project ID**: Used to associate each off-chain reserves program with its corresponding public explorer page

All authentication parameters are **pre-configured** within the delivered program. No additional setup is required on your side.

## What You Need to Configure

The client program is deployed **exclusively on your own server**, ensuring that you retain full control over all private credentials.

Before activating the program, you need to configure **read-only API keys** for accessing your off-chain asset data.

- Multiple off-chain data sources are supported
- Each data source may have multiple read-only API keys

## How the Client Program Works

Once the read-only API keys are configured, the client program is ready to run. After being started, it periodically completes off-chain reserve proofs according to the predefined execution cycle.

The execution flow is as follows:

1. **Initiate a zkTLS attestation process** to retrieve real-time asset balances from legitimate off-chain data source API endpoints.
2. **Validate the off-chain data source and hashed asset details** through Attestor nodes in the Primus Network, and propagate the validated data to the blockchain.
3. **Submit the retrieved data** to a verifiable computation backend (the zkVM network) via a TEE-assisted secure data processing channel.

These steps constitute the complete execution flow of the client program.

Within the zkVM network, asset data is processed according to the defined business logic, such as grouping by asset type or aggregating balances into a single reserve value. A zero-knowledge proof is then produced for the computed result.

Once this process is complete, the verified off-chain reserve values are publicly disclosed on the explorer page.

