
- **DAO Blueprint**: `proposal_nft_resource_manager`, `voter_nft_badge_resource_manager`, `admin_resource_manager` (3)
- **instantiate_dao method**, input: `owner_badge` (6)
- **create_proposal (10)**
    - Invoked after: a validation job will check `proposals_and_votes` hashmap, if `MAX_PROPOSALS_IN_VOTING` in the contract to ensure the maximum number of proposals in voting is not reached. If reached, don't call the component method and return the message to proposal_creator: `Sorry, maximum proposals have been reached for now, please try another time.`
    - input: `deposit_amount`
    - function: check `proposals_and_votes` hashmap, if `MAX_PROPOSALS_IN_VOTING` is reached, fail transaction and give a warning to the user.
    - output: return `proposal_nft_badge` to proposal_creator (to allow them to perform operations such as `amend` if the amendment period is open or `claim_deposits_minus_penalty`), store `proposal_creator_nft_badge` resource_address in `proposals_and_votes` hashmap (max length of 50, for example, if the limit is reached).

- **get_proposal_deposit_back (10)**
    - access control: user has `proposal_badge_nft`
    - input: `proposal_badge_nft` resource address
    - function: checks if the voting period has ended, if ended, returns deposit or deposit minus penalty (check the discussion section).
    - output: full or portion of amounts deposited

- **register_to_vote (7)**
    - input: amount of tokens to lock
    - output: gets a `voter_nft_badge` with NFT data containing:
        - `tokens_locked_amount`

- **withdraw_locked_token (7)**
    - input: amount of tokens to withdraw - should be more than 0 and less than or equal to the max amount of tokens locked on user NFT data
    - if all tokens are unlocked, the voter card is burnt, and the voter can no longer vote

- **cast_vote (5)**
    - access control: user has `voter_nft_badge`
    - input: vote with certain acceptable answers "yes, no, blank"?
    - function: updates the internal `proposal_and_votes` hashmap
    - output: N/A

- **Resolve Proposal Job (20)**
    - run every 24hrs
    - if there are no proposals in `proposal_and_votes`, just return
    - else go through `proposal_and_votes` hashmap
    - if the vote period has ended,
        - (still finalizing) resolve the proposal (see discussion below with pros and cons on how to resolve)

**Discussion** (2)

The following questions are organized based on the actors, resources, and constants

- **Actor: proposal_creator**
    - **Amendment process**
        - Do we need amendability? or is this just extra work
        - proposal_nft holder (creator) can invoke component to amend within amendable period (before voting begins)
        - what is amendable? Definition is a bit loose here....does this involve destroying current proposal and creating a new one?
    - **Cancel Proposal**
        - If the voting period is over, incurs a penalty, creator gets a portion of the deposit back
    - **Proposal Voted Down**
        - What happens now? Incur a penalty? return deposits back?

- **Actor: Voter**
    - Do we need weighted votes based on how much DAO resource is locked by the voter? Would that not skew votes all the time in favor of a whale and their colluders?
    - Can a voter withdraw a portion of the locked amount at any time, or is it all or nothing?
        - Assuming this would be right: If they voted with 20 Tokens locked, then withdraw 10, the vote still registers as having a 20-weight. If they vote on the same proposal or new proposals, the voting weight will be updated for the same or inserted if new as 10.

- **Resource: Vote**
    - what values we want for a vote: we can have `yes, no, blank`

- **Resource: Proposal**
    - Resolving a proposal can end up with a proposal whitelist being updated; this whitelist can reside in the following forms, the pros and cons are listed
        - In an iterable map inside the contract:
            - **pros**: readily decentralized and available, on a daily basis can be queried once and put in the cache (or manually be queried when needed and be put in the cache)
            - **cons**: can lead to state explosion, but a hard limit can be set in place **CRITICAL**: For testing limits we should know what the projected size of NFTs joining in 5 or 10 years is, querying it might cause a gas fee (needs to be explored, might be free of charge)
        - Maintain IPFs whitelist:
            - **pros**: Don't have to worry about **state explosion**, almost unlimited
            - **cons**: extra costs associated with IPFS, API usage, costs about API rate limiting and other backend maintenance, planning for availability issues (though the immutable and available as IPFS, the provider can go down potentially)

- **Constants**
    -  `MAX_PROPOSALS_IN_VOTING`: max number of proposals waiting to be voted on in the component's internal map called `proposals_and_votes`. suggested value `100`
    - `MAX_ACTIVE_PROPOSAL_WHITELIST`: max number of active proposals for Real256. See discussion; this is important, especially if the whitelist is maintained in the DAO component