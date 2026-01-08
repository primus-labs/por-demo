## Overview

This project verifies asset balances across multiple exchanges and account types, with support for **simultaneous zero-knowledge proofs over multiple accounts**.

It is composed of two main components:

* **[Client Program](./client/README.md)**: Provides user-facing functionality to configure exchanges, generate zkTLS attestations, submit tasks, and retrieve proof results.

* **[zkVM Program](./program/README.md)**: User-defined business logic compiled and executed inside a **zkVM** running within a **TEE**, responsible for verification, asset aggregation, and proof generation.

Together, these components enable privacy-preserving verification of exchange account balances using **zkTLS**, **TEE**, and **zero-knowledge proofs**.
