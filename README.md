# A Guide to DAO (Decentralized Autonomous Organization)

## Understanding Decentralized Autonomous Organization (DAO)

## High-Level Design Decisions for DAO Radix Components
**WARNING:** This document is a work in progress, and additional decisions may be added in the future.

- DAO designs vary in complexity based on factors such as:
  - `single proposal at a time` vs `multi-proposal at any period of time`: For example, Realm256 voters can choose between voting on a single NFT collection or multiple collections from different authors, provided that the voting periods overlap.
    - `single proposal`:
      - Pros: Simple to implement
      - Cons: Impractical for voters to stay online and vote for each new proposal; a proposal queue becomes necessary.
    - `multiple proposals`:
      - Pros: Voters can vote on multiple proposals as long as the voting period is open.
      - Cons: Must avoid `state explosion` (Subject to discussion: a. Are there that many proposals? b. Do we need to keep a history of which proposal got how much vote forever or can we just remove the proposal key after the voting round finishes? This is to prevent the component state from becoming too large.)
  - `Proposal LifeCycle`:
    - Some projects involve registration (where proposers deposit tokens to propose), approval by admin (as a gatekeeper, potential middleman attacks), user voting stage, and automatic actions based on vote results. Some steps in between can be further tweaked, and proposals can be removed from the component state object.
  - `DAO Resource`:
    - Do users claim DAO tokens from the dApp, or do they need to buy them from a DEX? Free claiming might be problematic, as individuals with different wallets could exploit this for free tokens.

Decisions made for now:

1. Let's go with the `multiple proposals` design.
2. Users have to buy DAO governance tokens from a DEX/CEX.

### Sequence Diagram (Actual diagram is a work in progress)
UserA: Buys DAO governance tokens from a Decentralized Exchange

Project Owners:
1. Invoke `Creates Proposal` and deposit a certain amount of XRD to prevent spam.
2. Admin approves or disapproves.
    - Approves: Proceed to step 3.
    - Disapproves: Proposer loses 50% (or 25%) of the locked amount.
3. Proposal is inserted into a Key-Value store of the DAO component with empty voting information.
4. Community/Admins: Off-chain users are informed about vote open and close days for various projects.
5. UserA: Attempts to vote
    - If possessing governance tokens, transfer them to the DAO component and claim NFT with weight calculated based on the number of governance tokens. Users may recast votes as long as the voting is open.
    - If satisfied with voting, can use the claim NFT to get back DAO governance tokens.
6. DAO Component: Once the right amount of votes is tallied for a proposal and the voting period has ended:
    - Perform an automatic action, e.g., send an NFT-Pass that gives Proposal Owners access to publish the project.
    - Remove proposal and voting info from DAO component state.

## Code Hardening and Design Considerations (to be continued)

Documentation Source [Code Hardening](https://docs.radixdlt.com/docs/code-hardening)

### Double Voting / Inflating Voting Weight
From the document:

```
In certain cases, special attention needs to be paid to the use of Proof amounts, especially in the context of applications that rely on such amounts for the casting of votes. Multiple proofs can be created of the same underlying assets.
```

1. Voters can cast votes with a maximum weight equal to the amount of DAO resources (e.g., a governance token) they possess.
2. They can vote for **multiple** proposals.

#### Potential Mishaps

1. If an ordinary proof of the amount of DAO resource is used, they can vote multiple times since, by proof, the tokens are not actually taken away from them.
2. They can create one proof of amount and then call the `vote` method multiple times, thus inflating the weight of their token and voting power.

#### Solution for the Mishap

1. Instead of sending proof of the resource, the voter will send in a bucket containing their DAO resource (e.g., governance token) to be held by the DAO.
2. In return, they will get a claim resource (e.g., a claim NFT or claim fungible tokens) that allows them to recast their vote (during the voting open period) and claim their DAO resource when the voting period is closed.

Refer to the code examples for both correct and incorrect DAO configurations.

### State Explosion

Documentation Source [Code Hardening](https://docs.radixdlt.com/docs/code-hardening)