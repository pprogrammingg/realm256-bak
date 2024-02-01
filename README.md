# A How to DAO

## What is Decentralized Autonomous Organization?


## High level DAO Radix component design decisions 
** WARNING ** this is an evolving document, so more decision might be added in the future

- DAO designs can range in complexity with regards to various factors. There are :
    - `single proposal at a time` vs `multi-proposal at any period of time`: for example Realm256 voters can choose between 1 NFT collection to vote on or multiple NFT collections from different authors, provided that the voting periods overlap.
        - `single proposal` 
            - pros : very simple to implement
            - cons : might be impractical to ask voters to keep online and vote for each new proposal and a proposal queue is then necessary as proposals won't move
        - `multlpe proposals`
            - pros: voters can vote on multiple proposals as long as the voting period for that proposal is open
            - cons: must take care to not have `state explosion` see below (Subject to discussion, a. Are there that many proposals? b. Do we neeed to keep a hisotry of which proposal got how much vote forever or can we just remove the proposal key after voting round finishes? fThis is so that the state of component does not get too big)
    - `Proposal LifeCycle`:
        - Some projects have register (where proposer deposits tokens to even propose), approval by admin (admin as gatekeepr, potential middleman attacks) --> enables proposal to be voted for should current time and date be between prosposal start and end dates, User Voting stage and automatic action based on vote result, Some of the steps in between can be tweaked further, remove proposal from component state object 
    - `DAO Resource`:
        - Do users claim DAO tokens from the dApp or they need ot buy from a DEX. Free claiming might be bad, as someone with different wallets will keep mintng free tokens.

The following are decided for now:

1. Let's go with `multiple proposals` design
2. Users have to buy DAO governance toekns from a DEX/CEX

### Sequence Diagram (Actual diagram is WIP)
UserA : Buys DAO governance token from a Decentralized Exchange

Project Owners:
1. invoke `Creates Proposal` and deposits certain amount of XRD. So that this does not become an spam habit to keep pushing proposals. 
2. Admin approves or disapproves.
    - Approves: go to step3.
    - Disapproves: proposer loses 50% of amount locked (or 25%)
3. Proposal is inserted in a Key-Value store of the DAO component with an empty voting information.
3. Community/Admins: off-chain users are communicated to about vote open and close days for various projects
4. UserA: Attemp vote
    - If posses governance token, transfer those the othe DAO component and get claim NFT with weight calculated based on number of governance tokens. Users may re-cast votes as long as voting is open 
    - If satisfied with voting can use claim NFT to get back DAO governance tokens
5. DAO Component: one the right amount of votes are tallied for a proposal and voting period has ended:
    - Perform an automatic action, e.g. send an NFT-Pass that gives Proposal Owners access to publish the project.
    - Remove proposal and voting info from DAO component state.

##  Code Hardening and Design Gotchas (to be continued)

Documentation Source [Code Hardening](https://docs.radixdlt.com/docs/code-hardening)


### Double Voting / Inflating Voting Weight
From the document
```
In certain cases, special attention needs to be paid to the use of Proof amounts, especially in the context of applications that rely on such amounts for the casting of votes. Multiple proofs can be created of the same underlying assets.

```

1. Voters can cast votes of a maximum weight equal to the amount of DAO resources (e.g. a governance token) they possess.
2. They can vote for **multiple** proposals.

#### Potential Mishap

1. If an ordinary proof of amount of DAO resource used, they can vote multiple times, since by proof the tokens are not actually taken away from them
2. They can create one proof of amount and then call the `vote` method multiple times thus inflating the wieght of their token and voting power.

#### Solution for the mishap

1. Instead of sending proof of resource, the voter will send in a bucket containing their DAO resource (e.g. governance token) to be held by the DAO
2. In return they will get a claim resources (e.g. a claim NFT or claim fungible tokens) that allows them to re-cast their vote (during voting open period) and be able to claim their DAO resource when voting period is closed.

- Refer to the code examples for both right and wrong DAO configurations



### State Explosion

Documentation Source [Code Hardening](https://docs.radixdlt.com/docs/code-hardening)









