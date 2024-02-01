# A Guide to DAO (Decentralized Autonomous Organization)

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


1. Proposal Creator: Invoke `Creates Proposal` and deposit a certain amount of XRD to prevent spam.
2. Component DAO: creates `proposal_NFT` badge with a unique ID. The state of the project is tracked within this NFT>
3. Admin: approves or disapproves the proposal
    - Approves: `proposal_NFT` ID is inserted into a Key-Value store of the DAO component with empty voting information. Status of `proposal_NFT` will change to `approved_for_voting`. Admin can set voting period or could automatically update on the NFT.
    - Disapproves: `proposal_NFT` gets updated with `proposal_rejected_by_admin`. Proposal creator can claim 75% of the initial deposit.
4. Communications Team: if `proposal_NFT` is `approved_for_voting`, the users are informed off-chain about vote open and close period for various projects.
5. UserA: Attempts to vote
    - If possessing governance tokens, transfer them to the DAO component and claim NFT with weight calculated based on the number of governance tokens. Users may recast votes as long as the voting is open and they have claim NFT.
    - Casting vote triggers `update_user_vote` on DAO component.
    - User may re-cast votes as long as the period is open.
    - Each re-cast will trigger `update_user_vote` on DAO Component again.
    - If voting period is over, user can use the claim NFT to get back DAO governance tokens.
6. DAO Component: Once the right amount of votes is tallied for a proposal and the voting period has ended:
    - If votes are in favour: update status of `proposal_NFT` to `proposal_ready_to_publish`
    - Else: update status of `proposal_NFT` to `proposal_did_not_get_enough_votes`
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

# Sequence Diagram
![DAO_Realm256](https://github.com/pprogrammingg/real256-bak/assets/29218920/cbace362-d976-4f0f-a986-eacb1c44b300)
