use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum Vote {
    Yes,
    No,
    Blank,
}

#[derive(ScryptoSbor)]
pub enum ProposalType {
    NftWhitelist,
    NftConfigChange,
    DaoConfigChange,
    Other
}

#[derive(ScryptoSbor)]
pub enum ProposalStatus {
    VotingStarted,
    VotingClosed
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ProposalData {
    proposal_type: ProposalType,
    data: HashMap<String, String>,
    status: ProposalStatus,
    voting_started_instant: Instant,
    voting_ended_instant: Instant,
    vote_stats: HashMap<ResourceAddress, Vote>,
}

pub struct DaoConfiguraiton {
    /**
     * quorum_treshold - 
     * Total REMs used to vote in favour must be above this ratio. 
     * REMs used is calculated from number of votes as well as rem_to_vote ratio.
     */
    quorum_treshold: Decimal,
    /**
     * rem_to_vote_ratio -
     * Determines the vote weight based on REMs used when voter registers, value of 1.0 means 1 REM is equal
     * to 1 vote weight.
     */
    rem_to_vote_ratio: Decimal, 
    /**
     * proposal_creation_min_rem_holding_ratio - 
     * Minimum amount of REM token needed to be held by proposal creator. 
     * Ratio represents REM Amount / Total REM Supply.
     */
    proposal_creation_min_rem_holding_ratio: Decimal,
    /**
     * proposal_creation_required_xrd_deposit_amount - 
     * Required XRD to deposit when creating proposal. 
     * Mainly used to prevent spamming proposal creation.
     */
    proposal_creation_xrd_fee: Decimal,
    /**
     * proposal_duration - 
     * Number of days the proposal is open from the voting_started_instant to
     * voting_ended_instant
     * 
     */
    proposal_period_in_days: u8,
}



#[blueprint]
mod dao {
    struct Dao {
       proposalsTable: HashMap<ResourceAddress, ProposalData>,


    }

    impl Dao {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_dao() -> Global<Dao> {

            Self {

            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }
    }
}
