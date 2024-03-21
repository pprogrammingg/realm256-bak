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
}

#[derive(ScryptoSbor)]
pub enum Status {
    VotingStarted,
    VotingClosed,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ProposalData {
    proposal_type: ProposalType,
    description: String,
    metadata: ProposalMetadataType,
    status: Status,
    voting_started_instant: Instant,
    voting_ended_instant: Instant,
    vote_results: HashMap<ResourceAddress, Vote>,
}

#[derive(ScryptoSbor)]
pub struct ProposalConfig {
    proposal_type: ProposalType,
    description: String,
    metadata: ProposalMetadataType,
}

#[derive(ScryptoSbor)]
pub enum ProposalMetadataType {
    NftWhiteListMetadata,
    NftConfigChangeMetadata,
    HashMapMetadata,
}

#[derive(ScryptoSbor)]
pub struct NftWhiteListMetadata {
    resource_address: ResourceAddress,
}
#[derive(ScryptoSbor)]
pub struct NftConfigChangeMetadata {
    resource_address: ResourceAddress,
    new_ips_url: String,
}

#[derive(ScryptoSbor)]
pub struct HashMapMetadata {
    data: HashMap<String, String>,
}

#[derive(ScryptoSbor)]
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
    proposal_period_in_days: i64,
}

#[blueprint]
mod dao {
    struct Dao {
        open_proposals_vault: NonFungibleVault, // vector holding open proposals
        dao_config: DaoConfiguraiton,
        proposal_nft_resource_manager: ResourceManager,
    }

    impl Dao {
        /* Instantiate DAO
         */
        pub fn instantiate_dao() -> Global<Dao> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Dao::blueprint_id());

            // ProposalNft ResourceManager
            let proposal_nft_resource_manager: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<ProposalData>(OwnerRole::None)
                    .metadata(metadata! {
                        init {
                            "name" => "Proposal NFT", locked;
                            "symbol" => "PROPOSAL_NFT", locked;
                        }
                    })
                    .mint_roles(mint_roles! {
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(deny_all);
                    })
                    .burn_roles(burn_roles! {
                        burner => rule!(require(global_caller(component_address)));
                        burner_updater => rule!(deny_all);
                    })
                    .create_with_no_initial_supply();

            let default_dao_config = DaoConfiguraiton {
                quorum_treshold: Decimal::from(200),
                rem_to_vote_ratio: Decimal::from(1),
                proposal_creation_min_rem_holding_ratio: dec!("0.001"),
                proposal_creation_xrd_fee: Decimal::from(0),
                proposal_period_in_days: 10i64,
            };

            Self {
                open_proposals_vault: NonFungibleVault::new(
                    proposal_nft_resource_manager.address(),
                ),
                dao_config: default_dao_config,
                proposal_nft_resource_manager,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }
        /**
         * create_proposal method
         * Inputs
         *  a) ProposalConfig proposal_config  - holds ProposalType and metadata
         * Performs the following:
         *  0. If ProposalsNftVault has reached full capacity exit with relevant error message
         *  1. Validates proposal_config metadata based proposal_type
         *  2. Set proposal_data as proposal_config fields in combination
         *  with calculated voting_started_instant, voting_ended_instant,
         *  status and an empty vote results object
         *  2. Mints a new PorposalNFT and sets the NFT data from the previous step
         *  3. Put the proposal in open_proposals_vault
         */
        pub fn create_proposal(&mut self, proposal_config: ProposalConfig) {
            // 0.
            // if (self.proposal_nft_vault) {
            // }

            // 1.
            let metadata = proposal_config.metadata;
            let match_result: Result<ProposalMetadataType, &'static str> =
                match proposal_config.proposal_type {
                    ProposalType::NftWhitelist => self.validate_nft_whitelist_proposal(metadata),
                    ProposalType::NftConfigChange => Ok(ProposalMetadataType::NftWhiteListMetadata),
                    ProposalType::DaoConfigChange => Ok(ProposalMetadataType::NftWhiteListMetadata),
                };

            assert!(
                match_result.is_ok(),
                "[CreateProposal] failed validation on input."
            );

            // 2.
            let metadata_type = match_result.unwrap();

            let current_instant = Clock::current_time_rounded_to_minutes();

            let proposal_data: ProposalData = ProposalData {
                proposal_type: proposal_config.proposal_type,
                description: proposal_config.description,
                metadata: metadata_type,
                status: Status::VotingStarted,
                voting_started_instant: current_instant,
                voting_ended_instant: current_instant
                    .add_days(self.dao_config.proposal_period_in_days)
                    .unwrap(),
                vote_results: HashMap::new(),
            };

            let proposal_nft: Bucket = self
                .proposal_nft_resource_manager
                .mint_ruid_non_fungible(proposal_data);

            // 3.
            self.open_proposals_vault
                .put(NonFungibleBucket(proposal_nft));
        }

        fn validate_nft_whitelist_proposal(
            &self,
            metadata: ProposalMetadataType,
        ) -> Result<ProposalMetadataType, &'static str> {
            match metadata {
                ProposalMetadataType::NftWhiteListMetadata => {
                    return Ok(ProposalMetadataType::NftWhiteListMetadata)
                }
                _ => return Err(
                    "Proposal to whitelist NFT must have metadata of type NftWhiteListMetadata.",
                ),
            }
        }
    }
}
