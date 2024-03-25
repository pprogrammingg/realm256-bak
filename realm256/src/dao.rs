use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum Vote {
    Yes,
    No,
    Blank,
}

#[derive(ScryptoSbor)]
pub enum ProposalDataTypes {
    NftCollectionWhiteListProposalData,
    NftCollectionConfigChangeProposalData,
    DaoConfigChangeProposalData,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NftCollectionWhiteListProposalData {
    common_data: CommonProposalData,
    metadata: NftCollectionWhiteListMetadata,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NftCollectionConfigChangeProposalData {
    common_data: CommonProposalData,
    metadata: NftCollectionConfigChangeMetadata,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct DaoConfigChangeProposalData {
    common_data: CommonProposalData,
    metadata: DaoConfigChangeMetadata,
}

#[derive(ScryptoSbor)]
pub struct NftCollectionWhiteListMetadata {
    resource_address: ResourceAddress,
}

#[derive(ScryptoSbor)]
pub struct NftCollectionConfigChangeMetadata {
    resource_address: ResourceAddress,
    update_fields: KeyValueStore<String, String>,
}

#[derive(ScryptoSbor)]
pub struct DaoConfigChangeMetadata {
    update_fields: KeyValueStore<String, String>,
}

#[derive(ScryptoSbor)]
pub enum Status {
    VotingStarted,
    VotingClosed,
    ProposalActionCompleted,
    ProposalRejected,
}

#[derive(ScryptoSbor)]
pub struct CommonProposalData {
    description: String,
    status: Status,
    voting_started_instant: Instant,
    voting_ended_instant: Instant,
    vote_results: HashMap<ResourceAddress, Vote>,
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
    /**
     * open_proposals_vault_max_capacity -
     * Number of open proposals that can exist, this is to avoid state explosion.
     *
     */
    open_proposals_vault_max_capacity: Decimal,
}

#[blueprint]
mod dao {
    struct Dao {
        nft_collection_whitelist_open_proposals_vault: NonFungibleVault,
        nft_collection_config_change_open_proposals_vault: NonFungibleVault,
        dao_config_change_open_proposals_vault: NonFungibleVault,
        nft_collection_white_list_proposal_nft_resource_manager: ResourceManager,
        nft_collection_config_change_proposal_nft_resource_manager: ResourceManager,
        dao_config_change_proposal_nft_resource_manager: ResourceManager,
        dao_config: DaoConfiguraiton,
        nft_whitelist_open_proposals_kv:
            KeyValueStore<ResourceAddress, NftCollectionWhiteListProposalData>,
    }

    impl Dao {
        /* Instantiate DAO
         */
        pub fn instantiate_dao() -> Global<Dao> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Dao::blueprint_id());

            // ProposalNft ResourceManager
            let nft_collection_white_list_proposal_nft_resource_manager: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<NftCollectionWhiteListProposalData>(
                    OwnerRole::None,
                )
                .metadata(metadata! {
                    init {
                        "name" => "NFT Whitelist Proposal", locked;
                        "symbol" => "NFT_WL_PROPOSAL", locked;
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

            let nft_collection_config_change_proposal_nft_resource_manager: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<NftCollectionConfigChangeProposalData>(
                    OwnerRole::None,
                )
                .metadata(metadata! {
                    init {
                        "name" => "NFT Config Change Proposal", locked;
                        "symbol" => "NFT_CFG_CHNG_PROPOSAL", locked;
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

            let dao_config_change_proposal_nft_resource_manager: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<NftCollectionConfigChangeProposalData>(
                    OwnerRole::None,
                )
                .metadata(metadata! {
                    init {
                        "name" => "DAO Config Change Proposal", locked;
                        "symbol" => "DAO_CFG_CHNG_PROPOSAL", locked;
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
                open_proposals_vault_max_capacity: Decimal::from(20),
            };

            Self {
                dao_config: default_dao_config,
                nft_collection_config_change_open_proposals_vault: NonFungibleVault::new(
                    nft_collection_white_list_proposal_nft_resource_manager.address(),
                ),
                nft_collection_whitelist_open_proposals_vault: NonFungibleVault::new(
                    nft_collection_config_change_proposal_nft_resource_manager.address(),
                ),
                dao_config_change_open_proposals_vault: NonFungibleVault::new(
                    dao_config_change_proposal_nft_resource_manager.address(),
                ),
                nft_collection_white_list_proposal_nft_resource_manager,
                nft_collection_config_change_proposal_nft_resource_manager,
                dao_config_change_proposal_nft_resource_manager,
                nft_whitelist_open_proposals_kv: KeyValueStore::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        /**
         * create_nft_collection_whitelist_proposal method
         * Args:
         *  a) metadata: NftCollectionWhiteListMetadata
         *  b) description: String
         *
         * Body:
         *   0. If proposal vault has reached full capacity exit with error
         *   1. Create common_data object for Proposal NFT
         *   2. Mint Proposal NFT with common_data and specific proposal metadata
         *   3. Put the proposal in the proposal vault
         */
        pub fn create_nft_collection_whitelist_proposal(
            &mut self,
            metadata: NftCollectionWhiteListMetadata,
            description: String,
        ) {
            // 0.
            let vault_amount = self.nft_collection_whitelist_open_proposals_vault.amount();
            assert!(
                self.nft_collection_whitelist_open_proposals_vault.amount()
                    <= self.dao_config.open_proposals_vault_max_capacity,
                "open_nft_collection_whitelist_proposal_vault capacity of {:?} has exceeded!",
                vault_amount
            );

            // 1.
            let common_data = self.create_common_proposal_data(description);

            // 2.
            let proposal_data = NftCollectionWhiteListProposalData {
                metadata,
                common_data,
            };

            let proposal_nft: Bucket = self
                .nft_collection_white_list_proposal_nft_resource_manager
                .mint_ruid_non_fungible(proposal_data);

            // Experimental
            //self.nft_whitelist_open_proposals_kv.insert(proposal_nft.resource_address(), proposal_data);

            // 3.
            self.nft_collection_whitelist_open_proposals_vault
                .put(NonFungibleBucket(proposal_nft));
        }

        /**
         * create_nft_collection_config_change_proposal method
         * Args:
         *  a) metadata: NftCollectionConfigChangeMetadata
         *  b) description: String
         *
         * Body:
         *   0. If proposal vault has reached full capacity exit with error
         *   1. calculate common_data for Proposal NFT
         *   2. Mint Proposal NFT with common_data and specific proposal metadata
         *   3. Put the proposal in proposal vault
         */
        pub fn create_nft_collection_config_change_proposal(
            &mut self,
            metadata: NftCollectionConfigChangeMetadata,
            description: String,
        ) {
            // 0.

            // 1.
            let common_data = self.create_common_proposal_data(description);

            // 2.
            let proposal_data = NftCollectionConfigChangeProposalData {
                metadata,
                common_data,
            };

            let proposal_nft: Bucket = self
                .nft_collection_config_change_proposal_nft_resource_manager
                .mint_ruid_non_fungible(proposal_data);

            // 3.
            self.nft_collection_config_change_open_proposals_vault
                .put(NonFungibleBucket(proposal_nft));
        }

        /**
         * create_dao_config_change_proposal method
         * Args:
         *  a) metadata: DaoConfigChangeMetadata
         *  b) description: String
         *
         * Body:
         *   0. If proposal vault has reached full capacity exit with error
         *   1. calculate common_data for Proposal NFT
         *   2. Mint Proposal NFT with common_data and specific proposal metadata
         *   3. Put the proposal in proposal vault
         */
        pub fn create_dao_config_change_proposal(
            &mut self,
            metadata: DaoConfigChangeMetadata,
            description: String,
        ) {
            // 0.

            // 1.
            let common_data = self.create_common_proposal_data(description);

            // 2.
            let proposal_data = DaoConfigChangeProposalData {
                metadata,
                common_data,
            };

            let proposal_nft: Bucket = self
                .dao_config_change_proposal_nft_resource_manager
                .mint_ruid_non_fungible(proposal_data);

            // 3.
            self.dao_config_change_open_proposals_vault
                .put(NonFungibleBucket(proposal_nft));
        }

        /**
         * create_common_proposal_data
         * takes in
         */
        fn create_common_proposal_data(&self, description: String) -> CommonProposalData {
            let current_instant = Clock::current_time_rounded_to_minutes();

            CommonProposalData {
                description: description,
                status: Status::VotingStarted,
                voting_started_instant: current_instant,
                voting_ended_instant: current_instant
                    .add_days(self.dao_config.proposal_period_in_days)
                    .unwrap(),
                vote_results: HashMap::new(),
            }
        }
    }
}
