use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use itertools::rev;
use mpl_metaplex::deprecated_state::WinningConfigItem;
use mpl_metaplex::instruction::MetaplexInstruction;
use mpl_metaplex::state::{NonWinningConstraint, WinningConfigType, WinningConstraint};
use serde::Serialize;
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98";

pub const NATIVE_BPF_LOADER_WRITE_TABLE_NAME: &str = "native_bpf_writes";
pub const METAPLEX_WITHDRAW_MASTER_EDITION_TABLE_NAME: &str = "metaplex_withdraw_master_edition";
pub const METAPLEX_REDEEMED_PARTICIPATION_BID_V2_TABLE_NAME: &str = "metaplex_redeemed_participation_bids_v2";
pub const METAPLEX_INIT_AUCTION_MANAGER_V2_TABLE_NAME: &str = "metaplex_init_auction_managers_v2";
pub const METAPLEX_VALIDATED_SAFETY_DEPOSIT_V2_TABLE_NAME: &str = "metaplex_validated_safety_deposits_v2";
pub const METAPLEX_REDEEMED_PARTICIPATION_BID_V3_TABLE_NAME: &str = "metaplex_redeemed_participation_bids_v3";
pub const METAPLEX_ENDED_AUCTION_TABLE_NAME: &str = "metaplex_ended_auctions";
pub const METAPLEX_SET_STORE_INDEX_TABLE_NAME: &str = "metaplex_set_store_indices";
pub const METAPLEX_SET_AUCTION_CACHE_TABLE_NAME: &str = "metaplex_set_auction_caches";

lazy_static! {
    pub static ref NATIVE_BPF_LOADER_WRITE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_bpf_write",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "program", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_WITHDRAW_MASTER_EDITION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_withdrawn_master_edition",
        "fields": [
            {"name": "auction_manager", "type": "string"},
            {"name": "safety_deposit_storage_account", "type": "string"},
            {"name": "new_mint_type_account", "type": "string"},
            {"name": "bid_redemption_key", "type": "string"},
            {"name": "safety_deposit_box_account", "type": "string"},
            {"name": "vault_account", "type": "string"},
            {"name": "safety_deposit_config_account", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "bidder_metadata", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "store", "type": "string"},
            {"name": "transfer_authority", "type": "string"},
            {"name": "accept_payment_account", "type": "string"},
            {"name": "potential_paying_token_account", "type": "string"},
            {"name": "prize_tracking_ticket", "type": "string"},
            {"name": "new_metadata_key", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "new_token_mint", "type": "string"},
            {"name": "edition_pda", "type": "string"},
            {"name": "new_mint_mint_authority", "type": "string"},
            {"name": "vault_token_metadata", "type": "string"},
            {"name": "auction_data_extended_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_REDEEMED_PARTICIPATION_BID_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_redeemed_participation_bid_v2",
        "fields": [
            {"name": "auction_manager", "type": "string"},
            {"name": "safety_deposit_storage_account", "type": "string"},
            {"name": "new_mint_type_account", "type": "string"},
            {"name": "bid_redemption_key", "type": "string"},
            {"name": "safety_deposit_box_account", "type": "string"},
            {"name": "vault_account", "type": "string"},
            {"name": "safety_deposit_config_account", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "bidder_metadata", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "store", "type": "string"},
            {"name": "transfer_authority", "type": "string"},
            {"name": "accept_payment_account", "type": "string"},
            {"name": "potential_paying_token_account", "type": "string"},
            {"name": "prize_tracking_ticket", "type": "string"},
            {"name": "new_metadata_key", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "new_token_mint", "type": "string"},
            {"name": "edition_pda", "type": "string"},
            {"name": "new_mint_mint_authority", "type": "string"},
            {"name": "vault_token_metadata", "type": "string"},
            {"name": "auction_data_extended_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_INIT_AUCTION_MANAGER_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_init_auction_manager_v2",
        "fields": [
            {"name": "auction_manager_account", "type": "string"},
            {"name": "auction_winner_token_type_tracker", "type": "string"},
            {"name": "combined_vault_account", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "payment_account", "type": "string"},
            {"name": "store", "type": "string"},
            {"name": "amount_type", "type": "int"},
            {"name": "length_type", "type": "int"},
            {"name": "max_ranges", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_VALIDATED_SAFETY_DEPOSIT_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_validated_safety_deposit_v2",
        "fields": [
            {"name": "win_index", "type": ["null","long"]},
            {"name": "key", "type": "int"},
            {"name": "order", "type": "long"},
            {"name": "winning_config_type", "type": "int"},
            {"name": "amount_type", "type": "int"},
            {"name": "length_type", "type": "int"},
            {"name": "amount_ranges", "type": "array", "items" : "long"]},
            {"name": "winning_constraint", "type": ["null","int"]},
            {"name": "non_winning_constraint", "type": ["null","int"]},
            {"name": "fixed_price", "type": ["null","long"]},
            {"name": "safety_deposit_config", "type": "string"},
            {"name": "auction_winner_token_type_tracker", "type": "string"},
            {"name": "auction_manager", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "og_authority_lookup", "type": "string"},
            {"name": "whitelisted_creators", "type": "string"},
            {"name": "auction_manager_store_key", "type": "string"},
            {"name": "safety_deposit_box", "type": "string"},
            {"name": "safety_deposit_box_storage", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "edition_record", "type": "string"},
            {"name": "vault_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "metadata_authority", "type": ["null","string"]},
            {"name": "payer", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_REDEEMED_PARTICIPATION_BID_V3_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_redeemed_participation_bid_v3",
        "fields": [
            {"name": "win_index", "type": ["null","long"]},
            {"name": "auction_manager", "type": "string"},
            {"name": "safety_deposit_storage_account", "type": "string"},
            {"name": "new_mint_type_account", "type": "string"},
            {"name": "bid_redemption_key", "type": "string"},
            {"name": "safety_deposit_box_account", "type": "string"},
            {"name": "vault_account", "type": "string"},
            {"name": "safety_deposit_config_account", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "bidder_metadata", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "store", "type": "string"},
            {"name": "transfer_authority", "type": "string"},
            {"name": "accept_payment_account", "type": "string"},
            {"name": "potential_paying_token_account", "type": "string"},
            {"name": "prize_tracking_ticket", "type": "string"},
            {"name": "new_metadata_key", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "new_token_mint", "type": "string"},
            {"name": "edition_pda", "type": "string"},
            {"name": "new_mint_mint_authority", "type": "string"},
            {"name": "vault_token_metadata", "type": "string"},
            {"name": "auction_data_extended_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_ENDED_AUCTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_ended_auction",
        "fields": [
            {"name": "auction_manager", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "auction_extended_data_account", "type": "string"},
            {"name": "auction_manager_authority", "type": "string"},
            {"name": "store_key", "type": "string"},
            {"name": "auction_program", "type": "string"},
            {"name": "reveal", "type": ["null","array"], "items" : "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_SET_STORE_INDEX_TABLE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_store_index",
        "fields": [
            {"name": "store_index", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "auction_cache", "type": "string"},
            {"name": "store_key", "type": "string"},
            {"name": "page", "type": "long"},
            {"name": "offset", "type": "long"},
            {"name": "auction_cache_above_current", "type": ["null","string"]},
            {"name": "auction_cache_below_current", "type": ["null","string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_SET_AUCTION_CACHE_TABLE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_auction_cache",
        "fields": [
            {"name": "auction_cache", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "safety_deposit_box_account", "type": "string"},
            {"name": "auction_manager", "type": "string"},
            {"name": "store_key", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum MetaplexMainDatum {
    DeprecatedInitAuctionManagerV1(InitAuctionManager),
    DeprecatedValidateSafetyDepositBoxV1(ValidateSafetyDepositBox),
    RedeemBid(RedeemedBid),
    RedeemFullRightsTransferBid(RedeemedFullRightsTransferBid),
    DeprecatedRedeemParticipationBid(RedeemedParticipationBid),
    StartAuction(StartedAuction),
    ClaimBid(ClaimedBid),
    EmptyPaymentAccount(EmptiedPaymentAccount),
    SetStore(SetStore),
    SetWhitelistedCreator(SetWhitelistedCreator),
    DeprecatedValidateParticipation(ValidatedParticipation),
    DeprecatedPopulateParticipationPrintingAccount(PopulatedParticipationPrintingAccount),
    RedeemUnusedWinningConfigItemsAsAuctioneer(RedeemedUnusedWinningConfigItemsAsAuctioneer),
    DecommissionAuctionManager(DecommissionedAuctionManager),
    RedeemPrintingV2Bid(RedeemedPrintingV2Bid),
    WithdrawMasterEdition(WithdrawnMasterEdition),
    DeprecatedRedeemParticipationBidV2(RedeemedParticipationBidV2),
    InitAuctionManagerV2(InitAuctionManagerV2),
    ValidateSafetyDepositBoxV2(ValidatedSafetyDepositBoxV2),
    RedeemParticipationBidV3(RedeemedParticipationBidV3),
    EndAuction(EndedAuction),
    SetStoreIndex(SetStoreIndex),
    SetAuctionCache(SetAuctionCache),
    SetStoreV2(SetStoreV2)
}

/// Struct tables
#[derive(Serialize)]
pub struct WinningItem {
    pub safety_deposit_box_index: i16,
    pub amount: i16,
    pub item_type: i16,
}

#[derive(Serialize)]
pub struct InitAuctionManager {
    /// Auction manager account with pda of ['metaplex', auction_key from auction referenced below]
    pub account: String,
    /// Combined vault account with authority set to auction manager account (this will be checked)
    /// Note in addition that this vault account should have authority set to this program's pda of ['metaplex', auction_key]
    pub vault_account: String,
    /// Auction with auctioned item being set to the vault given and authority set to this program's pda of ['metaplex', auction_key]
    pub auction: String,
    /// Signer of the instruction
    pub payer: String,
    /// Accept payment account of same token mint as the auction for taking payment for open editions, owner should be auction manager key
    pub payment_account: String,
    /// Store that this auction manager will belong to
    pub store: String,
    /// The safety deposit box index in the vault containing the winning items, in order of place
    /// The same index can appear multiple times if that index contains n tokens for n appearances (this will be checked)
    pub winning_config_items: Vec<WinningItem>,
    /// Setups:
    /// 0. Winners get participation + not charged extra
    /// 1. Winners dont get participation prize
    pub winning_constraint: Option<i16>,
    /// Setups:
    /// 0. Losers get prize for free
    /// 1. Losers get prize but pay fixed price
    /// 2. Losers get prize but pay bid price
    pub non_winning_constraint: Option<i16>,
    /// The safety deposit box index in the vault containing the template for the participation prize
    pub safety_deposit_box_index: Option<i16>,
    /// Setting this field disconnects the participation prizes price from the bid. Any bid you submit, regardless
    /// of amount, charges you the same fixed price.
    pub fixed_price: Option<i64>,
    pub authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ValidateSafetyDepositBox {

}

#[derive(Serialize)]
pub struct RedeemedBid {

}

#[derive(Serialize)]
pub struct RedeemedFullRightsTransferBid {

}

#[derive(Serialize)]
pub struct RedeemedParticipationBid {

}

#[derive(Serialize)]
pub struct StartedAuction {

}

#[derive(Serialize)]
pub struct ClaimedBid {

}

#[derive(Serialize)]
pub struct EmptiedPaymentAccount {

}

#[derive(Serialize)]
pub struct SetStore {

}

#[derive(Serialize)]
pub struct SetWhitelistedCreator {

}

#[derive(Serialize)]
pub struct ValidatedParticipation {

}

#[derive(Serialize)]
pub struct PopulatedParticipationPrintingAccount {

}

#[derive(Serialize)]
pub struct RedeemedUnusedWinningConfigItemsAsAuctioneer {

}

#[derive(Serialize)]
pub struct DecommissionedAuctionManager {

}

#[derive(Serialize)]
pub struct RedeemedPrintingV2Bid {

}

#[derive(Serialize)]
pub struct WithdrawnMasterEdition {
    pub auction_manager: String,
    pub safety_deposit_token_storage: String,
    /// Associated token account owned by auction manager authority of same mint as token storage account
    pub auction_manager_authority_ata: String,
    pub safety_deposit_box_account: String,
    pub vault_account: String,
    pub vault_fraction_mint: String,
    pub prize_tracking_ticket: String,
    pub vault_transfer_authority: String,
    pub auction: String,
    pub auction_data_extended: String,
    pub store: String,
    pub safety_deposit_config_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct RedeemedParticipationBidV2 {
    pub auction_manager: String,
    /// Safety deposit token storage account
    pub safety_deposit_storage_account: String,
    /// Account containing 1 token of your new mint type.
    /// MUST be an associated token account of pda [wallet, token program, mint] relative to ata program.
    pub new_mint_type_account: String,
    pub bid_redemption_key: String,
    pub safety_deposit_box_account: String,
    pub vault_account: String,
    /// Safety deposit config pda of ['metaplex', program id, auction manager, safety deposit]
    /// This account will only get used in the event this is an AuctionManagerV2
    pub safety_deposit_config_account: String,
    pub auction: String,
    pub bidder_metadata: String,
    pub bidder: String,
    pub payer: String,
    pub store: String,
    /// Transfer authority to move the payment in the auction's token_mint coin from the bidder account for the participation_fixed_price
    /// on the auction manager to the auction manager account itself.
    pub transfer_authority: String,
    pub accept_payment_account: String,
    /// The token account you will potentially pay for the open edition bid with if necessary.
    pub potential_paying_token_account: String,
    pub prize_tracking_ticket: String,
    pub new_metadata_key: String,
    pub new_edition: String,
    pub master_edition: String,
    pub new_token_mint: String,
    pub edition_pda: String,
    pub new_mint_mint_authority: String,
    pub vault_token_metadata: String,
    pub auction_data_extended_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct InitAuctionManagerV2 {
    pub auction_manager_account: String,
    pub auction_winner_token_type_tracker: String,
    pub combined_vault_account: String,
    pub auction: String,
    pub authority: String,
    pub payer: String,
    pub payment_account: String,
    pub store: String,
    pub amount_type: i16,
    pub length_type: i16,
    pub max_ranges: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ValidatedSafetyDepositBoxV2 {
    pub key: i16,
    /// safety deposit order
    pub order: i64,
    pub winning_config_type: i16,
    pub amount_type: i16,
    pub length_type: i16,
    pub amount_ranges: Vec<i64>,
    /// Setups:
    /// 0. Winners get participation + not charged extra
    /// 1. Winners dont get participation prize
    pub winning_constraint: Option<i16>,
    /// Setups:
    /// 0. Losers get prize for free
    /// 1. Losers get prize but pay fixed price
    /// 2. Losers get prize but pay bid price
    pub non_winning_constraint: Option<i16>,
    /// Setting this field disconnects the participation prizes price from the bid. Any bid you submit, regardless
    /// of amount, charges you the same fixed price.
    pub fixed_price: Option<i64>,
    /// We have this variable below to keep track in the case of the participation NFTs, whose
    /// income will trickle in over time, how much the artists have in the escrow account and
    /// how much would/should be owed to them if they try to claim it relative to the winning bids.
    /// It's  abit tougher than a straightforward bid which has a price attached to it, because
    /// there are many bids of differing amounts (in the case of GivenForBidPrice) and they dont all
    /// come in at one time, so this little ledger here keeps track.
    pub collected_to_accept_payment: Option<i64>,
    pub safety_deposit_config: String,
    pub auction_winner_token_type_tracker: String,
    pub auction_manager: String,
    pub metadata: String,
    pub og_authority_lookup: String,
    pub whitelisted_creators: String,
    pub auction_manager_store_key: String,
    pub safety_deposit_box: String,
    /// Safety deposit box storage account where the actual nft token is stored
    pub safety_deposit_box_storage: String,
    pub mint: String,
    pub edition_record: String,
    pub vault_account: String,
    pub authority: String,
    pub metadata_authority: Option<String>,
    pub payer: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct RedeemedParticipationBidV3 {
    pub win_index: Option<i64>,
    pub auction_manager: String,
    /// Safety deposit token storage account
    pub safety_deposit_storage_account: String,
    /// Account containing 1 token of your new mint type.
    /// MUST be an associated token account of pda [wallet, token program, mint] relative to ata program.
    pub new_mint_type_account: String,
    pub bid_redemption_key: String,
    pub safety_deposit_box_account: String,
    pub vault_account: String,
    /// Safety deposit config pda of ['metaplex', program id, auction manager, safety deposit]
    /// This account will only get used in the event this is an AuctionManagerV2
    pub safety_deposit_config_account: String,
    pub auction: String,
    pub bidder_metadata: String,
    pub bidder: String,
    pub payer: String,
    pub store: String,
    /// Transfer authority to move the payment in the auction's token_mint coin from the bidder account for the participation_fixed_price
    /// on the auction manager to the auction manager account itself.
    pub transfer_authority: String,
    pub accept_payment_account: String,
    /// The token account you will potentially pay for the open edition bid with if necessary.
    pub potential_paying_token_account: String,
    pub prize_tracking_ticket: String,
    pub new_metadata_key: String,
    pub new_edition: String,
    pub master_edition: String,
    pub new_token_mint: String,
    pub edition_pda: String,
    pub new_mint_mint_authority: String,
    pub vault_token_metadata: String,
    pub auction_data_extended_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct EndedAuction {
    pub auction_manager: String,
    pub auction: String,
    pub auction_extended_data_account: String,
    pub auction_manager_authority: String,
    pub store_key: String,
    pub auction_program: String,
    pub reveal: Option<Vec<i64>>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SetStoreIndex {
    pub store_index: String,
    pub payer: String,
    pub auction_cache: String,
    pub store_key: String,
    pub page: i64,
    pub offset: i64,
    pub auction_cache_above_current: Option<String>,
    pub auction_cache_below_current: Option<String>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SetAuctionCache {
    pub auction_cache: String,
    pub payer: String,
    pub auction: String,
    pub safety_deposit_box_account: String,
    pub auction_manager: String,
    pub store_key: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SetStoreV2 {
    pub store_key: String,
    pub store_config_key: String,
    pub admin: String,
    pub payer: String,
    pub public: bool,
    pub settings_uri: Option<String>,
    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    let mtp_ix_dr = MetaplexInstruction::try_from_slice(&instruction.data);

    return match mtp_ix_dr {
        Ok(ref bld) => {
            let deserialized_mtp_loader = bld.clone();
            let mut response: Vec<TableData> = Vec::new();
            return match deserialized_mtp_loader {
                MetaplexInstruction::DeprecatedInitAuctionManagerV1(auction_manager_settings) => {
                    let wc_item_sets: Vec<Vec<WinningConfigItem>> = auction_manager_settings.winning_configs
                        .into_iter()
                        .map(|wc| {
                            wc.items
                        })
                        .collect();

                    let mut wc_items =  Vec::new();
                    for wc_item_set in wc_item_sets {
                        for wci in wc_item_set {
                            wc_items.push(WinningItem {
                                safety_deposit_box_index: wci.safety_deposit_box_index as i16,
                                amount: wci.amount as i16,
                                item_type: match wci.winning_config_type {
                                    WinningConfigType::TokenOnlyTransfer => 0,
                                    WinningConfigType::FullRightsTransfer => 1,
                                    WinningConfigType::PrintingV1 => 2,
                                    WinningConfigType::PrintingV2 => 3,
                                    WinningConfigType::Participation => 4
                                }
                            });
                        }
                    }

                    let table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_WRITE_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_WRITE_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::DeprecatedInitAuctionManagerV1(
                            InitAuctionManager {
                                account: instruction.accounts[0].account.to_string(),
                                vault_account: instruction.accounts[1].account.to_string(),
                                auction: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[4].account.to_string(),
                                payment_account: instruction.accounts[5].account.to_string(),
                                store: instruction.accounts[6].account.to_string(),
                                winning_config_items: wc_items,
                                winning_constraint: if let Some(pc) = &auction_manager_settings.participation_config {
                                    Some(match pc.winner_constraint {
                                        WinningConstraint::NoParticipationPrize => 0,
                                        WinningConstraint::ParticipationPrizeGiven => 1
                                    })
                                } else {
                                    None
                                },
                                non_winning_constraint: if let Some(pc) = &auction_manager_settings.participation_config {
                                    Some(match pc.non_winning_constraint {
                                        NonWinningConstraint::NoParticipationPrize => 0,
                                        NonWinningConstraint::GivenForFixedPrice => 1,
                                        NonWinningConstraint::GivenForBidPrice => 2
                                    })
                                } else {
                                    None
                                },
                                safety_deposit_box_index: if let Some(pc) = &auction_manager_settings.participation_config {
                                    Some(pc.safety_deposit_box_index as i16)
                                } else {
                                    None
                                },
                                fixed_price: if let Some(pc) = &auction_manager_settings.participation_config {
                                    if let Some(fp) = pc.fixed_price {
                                        Some(fp as i64)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                },
                                authority: instruction.accounts[3].account.to_string(),
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::DeprecatedValidateSafetyDepositBoxV1 => {
                    msg!("Instruction: Deprecated Validate Safety Deposit Box V1");
                    process_deprecated_validate_safety_deposit_box_v1(program_id, accounts)
                }
                MetaplexInstruction::RedeemBid => {
                    msg!("Instruction: Redeem Normal Token Bid");
                    process_redeem_bid(program_id, accounts, None)
                }
                MetaplexInstruction::RedeemFullRightsTransferBid => {
                    msg!("Instruction: Redeem Full Rights Transfer Bid");
                    process_full_rights_transfer_bid(program_id, accounts, None)
                }
                MetaplexInstruction::DeprecatedRedeemParticipationBid => {
                    msg!("Instruction: Deprecated Redeem Participation Bid");
                    process_redeem_participation_bid(program_id, accounts, true, None)
                }
                MetaplexInstruction::StartAuction => {
                    msg!("Instruction: Start Auction");
                    process_start_auction(program_id, accounts)
                }
                MetaplexInstruction::ClaimBid => {
                    msg!("Instruction: Claim Bid");
                    process_claim_bid(program_id, accounts)
                }
                MetaplexInstruction::EmptyPaymentAccount(args) => {
                    msg!("Instruction: Empty Payment Account");
                    process_empty_payment_account(program_id, accounts, args)
                }
                MetaplexInstruction::SetStore(args) => {
                    msg!("Instruction: Set Store");
                    process_set_store(program_id, accounts, args.public)
                }
                MetaplexInstruction::SetStoreV2(args) => {
                    // msg!("Instruction: Set Store V2");
                    // process_set_store_v2(program_id, accounts, args.public, args.settings_uri)

                    let table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_WRITE_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_WRITE_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::SetStoreV2(
                            SetStoreV2 {
                                store_key: instruction.accounts[0].account.to_string(),
                                store_config_key: instruction.accounts[1].account.to_string(),
                                admin: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[3].account.to_string(),
                                public: args.public,
                                settings_uri: None,
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::SetWhitelistedCreator(args) => {
                    msg!("Instruction: Set Whitelisted Creator");
                    process_set_whitelisted_creator(program_id, accounts, args.activated)
                }
                MetaplexInstruction::DeprecatedValidateParticipation => {
                    msg!("Instruction: Deprecated Validate Open Edition");
                    process_deprecated_validate_participation(program_id, accounts)
                }
                MetaplexInstruction::DeprecatedPopulateParticipationPrintingAccount => {
                    msg!("Instruction: Deprecated Populate Participation Printing Account");
                    process_deprecated_populate_participation_printing_account(program_id, accounts)
                }
                MetaplexInstruction::RedeemUnusedWinningConfigItemsAsAuctioneer(args) => {
                    msg!("Instruction: Redeem Unused Winning Config Items As Auctioneer");
                    process_redeem_unused_winning_config_items_as_auctioneer(program_id, accounts, args)
                }
                MetaplexInstruction::DecommissionAuctionManager => {
                    msg!("Instruction: Decomission Auction Manager");
                    process_decommission_auction_manager(program_id, accounts)
                }
                MetaplexInstruction::RedeemPrintingV2Bid(args) => {
                    msg!("Instruction: Redeem Printing V2 Bid");
                    process_redeem_printing_v2_bid(
                        program_id,
                        accounts,
                        args.edition_offset,
                        args.win_index,
                    )
                }
                MetaplexInstruction::WithdrawMasterEdition => {
                    // msg!("Instruction: Withdraw Master Edition");
                    // process_withdraw_master_edition(program_id, accounts)

                    let table_data = TableData {
                        schema: (*METAPLEX_WITHDRAW_MASTER_EDITION_SCHEMA).clone(),
                        table_name: METAPLEX_WITHDRAW_MASTER_EDITION_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::WithdrawMasterEdition(
                            WithdrawnMasterEdition {
                                auction_manager: instruction.accounts[0].account.to_string(),
                                safety_deposit_token_storage: instruction.accounts[1].account.to_string(),
                                auction_manager_authority_ata: instruction.accounts[2].account.to_string(),
                                safety_deposit_box_account: instruction.accounts[3].account.to_string(),
                                vault_account: instruction.accounts[4].account.to_string(),
                                vault_fraction_mint: instruction.accounts[5].account.to_string(),
                                prize_tracking_ticket: instruction.accounts[6].account.to_string(),
                                vault_transfer_authority: instruction.accounts[7].account.to_string(),
                                auction: instruction.accounts[8].account.to_string(),
                                auction_data_extended: instruction.accounts[9].account.to_string(),
                                store: instruction.accounts[12].account.to_string(),
                                safety_deposit_config_account: instruction.accounts[14].account.to_string(),
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::DeprecatedRedeemParticipationBidV2 => {
                    // msg!("Instruction: Deprecated Redeem Participation Bid V2");
                    // process_redeem_participation_bid(program_id, accounts, false, None)

                    let table_data = TableData {
                        schema: (*METAPLEX_REDEEMED_PARTICIPATION_BID_V2_SCHEMA).clone(),
                        table_name: METAPLEX_REDEEMED_PARTICIPATION_BID_V2_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::DeprecatedRedeemParticipationBidV2(
                            RedeemedParticipationBidV2 {
                                auction_manager: instruction.accounts[0].account.to_string(),
                                safety_deposit_storage_account: instruction.accounts[1].account.to_string(),
                                new_mint_type_account: instruction.accounts[2].account.to_string(),
                                bid_redemption_key: instruction.accounts[3].account.to_string(),
                                safety_deposit_box_account: instruction.accounts[4].account.to_string(),
                                vault_account: instruction.accounts[5].account.to_string(),
                                safety_deposit_config_account: instruction.accounts[6].account.to_string(),
                                auction: instruction.accounts[7].account.to_string(),
                                bidder_metadata: instruction.accounts[8].account.to_string(),
                                bidder: instruction.accounts[9].account.to_string(),
                                payer: instruction.accounts[10].account.to_string(),
                                store: instruction.accounts[14].account.to_string(),
                                transfer_authority: instruction.accounts[17].account.to_string(),
                                accept_payment_account: instruction.accounts[18].account.to_string(),
                                potential_paying_token_account: instruction.accounts[19].account.to_string(),
                                prize_tracking_ticket: instruction.accounts[20].account.to_string(),
                                new_metadata_key: instruction.accounts[21].account.to_string(),
                                new_edition: instruction.accounts[22].account.to_string(),
                                master_edition: instruction.accounts[23].account.to_string(),
                                new_token_mint: instruction.accounts[24].account.to_string(),
                                edition_pda: instruction.accounts[25].account.to_string(),
                                new_mint_mint_authority: instruction.accounts[26].account.to_string(),
                                vault_token_metadata: instruction.accounts[27].account.to_string(),
                                auction_data_extended_account: instruction.accounts[28].account.to_string(),
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::InitAuctionManagerV2(args) => {
                    // msg!("Instruction: Init Auction Manager V2");
                    // process_init_auction_manager_v2(
                    //     program_id,
                    //     accounts,
                    //     args.amount_type,
                    //     args.length_type,
                    //     args.max_ranges,
                    // )
                    // msg!("Instruction: End auction");
                    // process_end_auction(program_id, accounts, args)

                    let table_data = TableData {
                        schema: (*METAPLEX_INIT_AUCTION_MANAGER_V2_SCHEMA).clone(),
                        table_name: METAPLEX_INIT_AUCTION_MANAGER_V2_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::InitAuctionManagerV2(
                            InitAuctionManagerV2 {
                                auction_manager_account: instruction.accounts[0].account.to_string(),
                                auction_winner_token_type_tracker: instruction.accounts[1].account.to_string(),
                                combined_vault_account: instruction.accounts[2].account.to_string(),
                                auction: instruction.accounts[3].account.to_string(),
                                authority: instruction.accounts[4].account.to_string(),
                                payer: instruction.accounts[5].account.to_string(),
                                payment_account: instruction.accounts[6].account.to_string(),
                                store: instruction.accounts[7].account.to_string(),
                                amount_type: args.amount_type as i16,
                                length_type: args.length_type as i16,
                                max_ranges: args.max_ranges as i64,
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::ValidateSafetyDepositBoxV2(safety_deposit_config) => {
                    // msg!("Instruction: Validate Safety Deposit Box V2");
                    // process_validate_safety_deposit_box_v2(program_id, accounts, safety_deposit_config)

                    let table_data = TableData {
                        schema: (*METAPLEX_VALIDATED_SAFETY_DEPOSIT_V2_SCHEMA).clone(),
                        table_name: METAPLEX_VALIDATED_SAFETY_DEPOSIT_V2_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::ValidateSafetyDepositBoxV2(
                            ValidatedSafetyDepositBoxV2 {
                                key: safety_deposit_config.key as i16,
                                order: safety_deposit_config.order as i64,
                                winning_config_type: safety_deposit_config.winning_config_type as i16,
                                amount_type: safety_deposit_config.amount_type as i16,
                                length_type: safety_deposit_config.length_type as i16,
                                amount_ranges: vec![safety_deposit_config.amount_ranges[0].0 as i64,
                                                    safety_deposit_config.amount_ranges[0].1 as i64,
                                                    safety_deposit_config.amount_ranges[1].0 as i64,
                                                    safety_deposit_config.amount_ranges[1].1 as i64],
                                winning_constraint: if let Some(pc) = &safety_deposit_config.participation_config {
                                    Some(pc.winner_constraint as i16)
                                } else {
                                    None
                                },
                                non_winning_constraint: if let Some(pc) = &safety_deposit_config.participation_config {
                                    Some(pc.non_winning_constraint as i16)
                                } else {
                                    None
                                },
                                fixed_price: if let Some(pc) = &safety_deposit_config.participation_config {
                                    if let Some(fp) = pc.fixed_price {
                                        Some(fp as i64)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                },
                                collected_to_accept_payment: if let Some(ps) = &safety_deposit_config.participation_state {
                                    Some(ps.collected_to_accept_payment as i64)
                                } else {
                                    None
                                },
                                safety_deposit_config: instruction.accounts[0].account.to_string(),
                                auction_winner_token_type_tracker: instruction.accounts[1].account.to_string(),
                                auction_manager: instruction.accounts[2].account.to_string(),
                                metadata: instruction.accounts[3].account.to_string(),
                                og_authority_lookup: instruction.accounts[4].account.to_string(),
                                whitelisted_creators: instruction.accounts[5].account.to_string(),
                                auction_manager_store_key: instruction.accounts[6].account.to_string(),
                                safety_deposit_box: instruction.accounts[7].account.to_string(),
                                safety_deposit_box_storage: instruction.accounts[8].account.to_string(),
                                mint: instruction.accounts[9].account.to_string(),
                                edition_record: instruction.accounts[10].account.to_string(),
                                vault_account: instruction.accounts[11].account.to_string(),
                                authority: instruction.accounts[12].account.to_string(),
                                metadata_authority: if instruction.accounts.len() < 18 {
                                    None
                                } else {
                                    Some(instruction.accounts[13].account.to_string())
                                },
                                payer: if instruction.accounts.len() < 18 {
                                    instruction.accounts[13].account.to_string()
                                } else {
                                    instruction.accounts[14].account.to_string()
                                },
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::RedeemParticipationBidV3(args) => {
                    // msg!("Instruction: Redeem Participation Bid V3");
                    // process_redeem_participation_bid(program_id, accounts, false, args.win_index)

                    let table_data = TableData {
                        schema: (*METAPLEX_REDEEMED_PARTICIPATION_BID_V3_SCHEMA).clone(),
                        table_name: METAPLEX_REDEEMED_PARTICIPATION_BID_V3_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::RedeemParticipationBidV3(
                            RedeemedParticipationBidV3 {
                                win_index: if let Some(win_index) = args.win_index {
                                    Some(win_index as i64)
                                } else {
                                    None
                                },
                                auction_manager: instruction.accounts[0].account.to_string(),
                                safety_deposit_storage_account: instruction.accounts[1].account.to_string(),
                                new_mint_type_account: instruction.accounts[2].account.to_string(),
                                bid_redemption_key: instruction.accounts[3].account.to_string(),
                                safety_deposit_box_account: instruction.accounts[4].account.to_string(),
                                vault_account: instruction.accounts[5].account.to_string(),
                                safety_deposit_config_account: instruction.accounts[6].account.to_string(),
                                auction: instruction.accounts[7].account.to_string(),
                                bidder_metadata: instruction.accounts[8].account.to_string(),
                                bidder: instruction.accounts[9].account.to_string(),
                                payer: instruction.accounts[10].account.to_string(),
                                store: instruction.accounts[14].account.to_string(),
                                transfer_authority: instruction.accounts[17].account.to_string(),
                                accept_payment_account: instruction.accounts[18].account.to_string(),
                                potential_paying_token_account: instruction.accounts[19].account.to_string(),
                                prize_tracking_ticket: instruction.accounts[20].account.to_string(),
                                new_metadata_key: instruction.accounts[21].account.to_string(),
                                new_edition: instruction.accounts[22].account.to_string(),
                                master_edition: instruction.accounts[23].account.to_string(),
                                new_token_mint: instruction.accounts[24].account.to_string(),
                                edition_pda: instruction.accounts[25].account.to_string(),
                                new_mint_mint_authority: instruction.accounts[26].account.to_string(),
                                vault_token_metadata: instruction.accounts[27].account.to_string(),
                                auction_data_extended_account: instruction.accounts[28].account.to_string(),
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::EndAuction(args) => {
                    // msg!("Instruction: End auction");
                    // process_end_auction(program_id, accounts, args)

                    let table_data = TableData {
                        schema: (*METAPLEX_ENDED_AUCTION_SCHEMA).clone(),
                        table_name: METAPLEX_ENDED_AUCTION_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::EndAuction(
                            EndedAuction {
                                auction_manager: instruction.accounts[0].account.to_string(),
                                auction: instruction.accounts[1].account.to_string(),
                                auction_extended_data_account: instruction.accounts[2].account.to_string(),
                                auction_manager_authority: instruction.accounts[3].account.to_string(),
                                store_key: instruction.accounts[4].account.to_string(),
                                auction_program: instruction.accounts[5].account.to_string(),
                                reveal: if let Some(reveal) = args.reveal {
                                    Some(vec![reveal.0 as i64, reveal.1 as i64])
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::SetStoreIndex(args) => {
                    // msg!("Instruction: Set Store Index");
                    // process_set_store_index(program_id, accounts, args)

                    let table_data = TableData {
                        schema: (*METAPLEX_SET_STORE_INDEX_TABLE_SCHEMA).clone(),
                        table_name: METAPLEX_SET_STORE_INDEX_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::SetStoreIndex(
                            SetStoreIndex {
                                store_index: instruction.accounts[0].account.to_string(),
                                auction_cache: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[1].account.to_string(),
                                store_key: instruction.accounts[3].account.to_string(),
                                page: args.page as i64,
                                offset: args.offset as i64,
                                auction_cache_above_current: if instruction.accounts.len() > 6 {
                                    Some(instruction.accounts[6].account.to_string())
                                } else {
                                    None
                                },
                                auction_cache_below_current: if instruction.accounts.len() > 7 {
                                    Some(instruction.accounts[7].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
                MetaplexInstruction::SetAuctionCache => {
                    // msg!("Instruction: Set Auction Cache");
                    // process_set_auction_cache(program_id, accounts)
                    let table_data = TableData {
                        schema: (*METAPLEX_SET_AUCTION_CACHE_TABLE_SCHEMA).clone(),
                        table_name: METAPLEX_SET_AUCTION_CACHE_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::Metaplex(MetaplexMainDatum::SetAuctionCache(
                            SetAuctionCache {
                                auction_cache: instruction.accounts[0].account.to_string(),
                                payer: instruction.accounts[1].account.to_string(),
                                auction: instruction.accounts[2].account.to_string(),
                                safety_deposit_box_account: instruction.accounts[3].account.to_string(),
                                auction_manager: instruction.accounts[4].account.to_string(),
                                store_key: instruction.accounts[5].account.to_string(),
                                timestamp: instruction.timestamp,
                            }))]
                    };

                    response.push(table_data);

                    Some(response)
                }
            };
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", instruction.program, err);

            None
        }
    };
}