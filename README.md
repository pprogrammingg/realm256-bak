# Realm DAO 

## **WARNING:** This document is a work in progress, and additional decisions may be added in the future.

# Table of Contents
- [Component Overview](#component-overview)
  - [Main methods](#main-methods)
- [Package Deployment](#package-deployment)
- [dApp Instantiation](#dapp-instantiation)

# Sequence Diagram
![DAO_Realm256](DAO_Realm256.svg)

# Deployment

# Component Overview

# Package Deployment

1. Clone repo [realm256]()
2. Change to [realm256]() directory where `cargo.toml`is.
3. **Delete Cached Artifacts**: If there is a target folder,make sure there are no cached. For this, delete `./target/wasm32-unknown-unknown/release/realm256.wasm` and `./target/wasm32-unknown-unknown/release/realm256.rpd`. 
4. Execute `scrypto build`, new `.wasm` and `.rpd` should be created.
5. **Developer Console**: Navigate to developer console, either [Stokenet](https://stokenet-console.radixdlt.com/) or [Mainnet](https://console.radixdlt.com/).
6. **Connect Wallet**: from top right corner, click `Connect`. Inside wallet choose perona and account. 
7. **Prepare to Deploy**: From the left navigation menu, click `Deploy Package`. Here,
upload `.wasm` and `.rpd` files created in the previous steps. A deploy package badge can be either created on the fly or an existing one can be used. This `package deployment badge`(essentially a funginle or non-fungible) should not to be confused with `dApp ownder badge` which is a badge we use to give our dApp an owner.
8. **Deploy Package**: Once the `.wasm` and `.rpd` files as well as `package deployment badge` are selected, press `submit`.**NOTE**: Assuming that Radix standard wallet is used, make sure a fee payer is selected, if not click `customize`, then `Select Fee Payer` and then select an account to pay the trnasaction fee. **Note**  if testing on `Stokenet` give any account some free XRD by clicking the 3 dots (`...`) on top of account screen, then `Dev Preferences` and finally `Get XRD Test Tokens`.
9. If success, paste the created package address under `config/global_config.json` key `package_info.package_address`.These values are important for the instantiation step.