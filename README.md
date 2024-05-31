<h1 align="center" style="font-size: 200px;">
  <strong>GECKO</strong>
</h1>
<p align="center">
    <br />
        <img src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/273f6ecc-8cb1-4d73-88d1-320513053c69" width="225" alt=""/></a>
    <br />
</p>

<p align="center"><strong>The first Clarity static analyser that finds bugs in your smart contracts


### What is Gecko?
Gecko is an open-source Clarity static analysis tool written in Rust. It currently detects two vulnerabilities: unsafe inputs and authentication via `tx-sender`. When it finds these vulnerabilities, Gecko provides visual information about the bug. It also features a web UI for easy contract testing. Developers can upload and test their own contracts or input deployed contracts for Gecko to scan. This makes it useful for both pre and post-deployment testing. Gecko helps developers find vulnerabilities and improve their code comprehension. Additionally, it reassures users that the deployed contracts they interact with are safe.

### Images
<img width="1510" alt="Screenshot 2024-05-31 at 08 04 45" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/7b66a7f0-8017-4e7c-93a7-4c3eadb07cb9">
<img width="1508" alt="Screenshot 2024-05-31 at 08 04 52" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/f2a983a0-67a5-4a57-905b-42a764142673">
<img width="1510" alt="Screenshot 2024-05-31 at 08 05 19" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/de7d1ea6-1e00-4875-9f02-7efbaed65592">
<img width="1509" alt="Screenshot 2024-05-31 at 08 05 29" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/9b81344e-758b-4c5c-a2d1-dd178d0d7796">
<img width="1512" alt="Screenshot 2024-05-31 at 07 45 39" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/f2a17c7f-0487-430f-a6e9-338fd1cf86c4">
<img width="1512" alt="Screenshot 2024-05-31 at 07 45 58" src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/f635405e-01e6-4188-923c-610ebf5d1ace">


- [Demo (finding vulnearbility in previous zest-protocol contract)](https://youtu.be/1UTiEWyAK4Q)
- [Deck](https://www.canva.com/design/DAGGzprfItY/EBvcoKtM9bMAdMIbDPgFkQ/edit?utm_content=DAGGzprfItY&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton)


### Features
Currently Gecko only supports checking for unsafe inputs and for vulnearbilities that occur when `tx-sender` is used for authentication. These were chosen as they were the most common vulnearbilities that can be found in clarity contracts see this [report](https://www.coinfabrik.com/blog/tx-sender-in-clarity-smart-contracts-is-not-adviced/). The full list of vulnerabilities that will be added can be found [here](https://github.com/Gecko-Security/Gecko-Clarity/tree/main?tab=readme-ov-file#vulnerability-detectors). 

### How Gecko's Static Analysis Works
1. We parse the Clarity code into a structure that Gecko can understand, this is called an Abstract Syntax Tree (AST). It represents the hierarchical structure of the code. We use the [Clarity Contract Analysis Crate](https://docs.rs/stacks-codec/latest/stacks_codec/clarity/vm/analysis/types/struct.ContractAnalysis.html), which converts Clarity code into an AST and other metadata. This is the main entrypoint for Gecko.
2. We then define a struct called [Gecko](https://github.com/Gecko-Security/Gecko-Clarity/blob/main/static/static/src/main.rs#L52), which implements the [`ast_visitor`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_ast/visit/trait.Visitor.html) crate used to traverse each node and understand the behaviour of the code.
3. Taint analysis is used to track the flow of potentially unsafe data through the program and locate bugs and vulnearbilities. This involves defining the vulnearbility detectors as invariants and tracking the data to ensure it is properly checked or sanitized.
4. As Gecko traverses the tree it propagates this taint to other nodes that depend on these.
5. Once the traversal is complete messages are displayed about issues found including the location of the bug in source.

<p align="center">
    <br />
        <img src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/05d5d059-2e65-448e-b154-9818a72b3408" width="800" alt=""/></a>
    <br />
</p>

_Gecko Technical Architecture_


<p align="center">
    <br />
        <img src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/b651810b-6db5-457c-88d1-7ee5feb6dc01" width="800" alt=""/></a>
    <br />
</p>

_Example of a traversal of AST_


###  Vulnerability Detectors
The following is a table of vulnearbility detectors supported by Gecko and future detectors that will be added when as the AST is impoved and dynamic analysis such as fuzzing is added. 

The aim is to create a set of real-life vulnearbilities and examples that will not only serve as a robust development template but also help identify good and bad parctices in Clarity contract development. Contibution of adding new vulnearbilities or examples is welcome. 


| ✔️  | Vulnerability                                                                              | Example/Description                                                                                                                                                                                                                                                                                                                                                                                             |
| --- | ------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔️  | authentication via `tx-sender`                                                             | [Report](https://www.coinfabrik.com/blog/tx-sender-in-clarity-smart-contracts-is-not-adviced/) , Example: [Arkadiko](https://github.com/arkadiko-dao/arkadiko/blob/cbb0ed52fd06780f3d167e94138a6ad51b44cc44/clarity/contracts/wstx-token.clar#L55)                                                                                                                                                              |
| ✔️  | untrusted actions on Stacks wallets (`stx-burn?`, `stx-transfer?`)                         |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | untrusted actions on fungible tokens (`ft-burn?`, `ft-mint?`, `ft-transfer?`)              |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | untrusted actions on non-fungible tokens (`nft-burn?`, `nft-mint?`, `nft-transfer?`)       |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | untrusted actions on persisted data (`map-delete?`, `map-insert?`, `map-set?`, `var-set?`) |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | calls to private functions                                                                 |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | return values                                                                              |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | dynamic contract calls (through traits)                                                    |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | block time assumption broken on nakamoto release                                           | Farming and stacking core contracts assume block time for the calculation of epoch lengths. However, this assumption is expected to be modified in the next Stacks upgrade (Nakamoto Release), which will reduce block time.                                                                                                                                                                                    |
|     | rounding errors                                                                            |                                                                                                                                                                                                                                                                                                                                                                                                                 |
|     | panicking on possible error                                                                | Using `unwrap-panic` results in the transaction being finished because of a runtime error when the provided value is an error or a none. The runtime error does not allow the caller to handle that error and act in response. Example: [Zest Protocol](https://github.com/Zest-Protocol/zest-contracts/blob/dae42d8d6aa4710cab95bd44717a9dda40f2bd2e/onchain/contracts/borrow/vaults/pool-0-reserve.clar#L225) |
|     | `as-contract` call to unverified principal                                                 | Enclosing a contract call in an `as-contract` expression makes this internal call to be made on behalf of the caller contract. In the example the `tx-sender` value is changed to this caller contract. Example: [Zest Protocol](https://github.com/Zest-Protocol/zest-contracts/blob/dae42d8d6aa4710cab95bd44717a9dda40f2bd2e/onchain/contracts/borrow/vaults/pool-0-reserve.clar#L1074)                       |
|     | signature replay in oracle                                                                 | Example shows oracle prices are updated with a multi-signature scheme. However, besides validating the signature's content and verifying the signer, the function does not check whether the signatures were already used. Example: [Arkadiko](https://github.com/arkadiko-dao/arkadiko/blob/cbb0ed52fd06780f3d167e94138a6ad51b44cc44/clarity/contracts/arkadiko-oracle-v2-2.clar#L96)                          |
|     | race condition                                                                             | Example shows the interaction between the burning of USDA tokens and the subsequent adjustment of the fragments-per-token variable in the liquidity contract results in lost rewards for the users. Example: [Arkadiko](https://github.com/arkadiko-dao/arkadiko/blob/cbb0ed52fd06780f3d167e94138a6ad51b44cc44/clarity/contracts/vaults-v2/arkadiko-vaults-pool-liq-v1-1.clar#L240)                             |
|     | free front-running                                                                         | Example shows fees are charged when minting USDA through `open-vault()` or `update-vault()`, if the user adds collateral to the vault. The minting fee is set in the function `set-mint-fee()`. Example: [Arkadiko](https://github.com/arkadiko-dao/arkadiko/blob/cbb0ed52fd06780f3d167e94138a6ad51b44cc44/clarity/contracts/vaults-v2/arkadiko-vaults-operations-v1-1.clar#L73)                                |
