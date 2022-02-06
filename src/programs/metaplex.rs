use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use mpl_metaplex::deprecated_state::WinningConfigItem;
use mpl_metaplex::instruction::MetaplexInstruction;
use mpl_metaplex::state::{NonWinningConstraint, WinningConfigType, WinningConstraint};
use serde::Serialize;
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98";

pub const NATIVE_BPF_LOADER_WRITE_TABLE_NAME: &str = "native_bpf_writes";

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

}

#[derive(Serialize)]
pub struct RedeemedParticipationBidV2 {

}

#[derive(Serialize)]
pub struct InitAuctionManagerV2 {

}

#[derive(Serialize)]
pub struct ValidatedSafetyDepositBoxV2 {

}

#[derive(Serialize)]
pub struct RedeemedParticipationBidV3 {

}

#[derive(Serialize)]
pub struct EndedAuction {

}

#[derive(Serialize)]
pub struct SetStoreIndex {

}

#[derive(Serialize)]
pub struct SetAuctionCache {

}

#[derive(Serialize)]
pub struct SetStoreV2 {

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
                    msg!("Instruction: Set Store V2");
                    process_set_store_v2(program_id, accounts, args.public, args.settings_uri)
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
                    msg!("Instruction: Withdraw Master Edition");
                    process_withdraw_master_edition(program_id, accounts)
                }
                MetaplexInstruction::DeprecatedRedeemParticipationBidV2 => {
                    msg!("Instruction: Deprecated Redeem Participation Bid V2");
                    process_redeem_participation_bid(program_id, accounts, false, None)
                }
                MetaplexInstruction::InitAuctionManagerV2(args) => {
                    msg!("Instruction: Init Auction Manager V2");
                    process_init_auction_manager_v2(
                        program_id,
                        accounts,
                        args.amount_type,
                        args.length_type,
                        args.max_ranges,
                    )
                }
                MetaplexInstruction::ValidateSafetyDepositBoxV2(safety_deposit_config) => {
                    msg!("Instruction: Validate Safety Deposit Box V2");
                    process_validate_safety_deposit_box_v2(program_id, accounts, safety_deposit_config)
                }
                MetaplexInstruction::RedeemParticipationBidV3(args) => {
                    msg!("Instruction: Redeem Participation Bid V3");
                    process_redeem_participation_bid(program_id, accounts, false, args.win_index)
                }
                MetaplexInstruction::EndAuction(args) => {
                    msg!("Instruction: End auction");
                    process_end_auction(program_id, accounts, args)
                }
                MetaplexInstruction::SetStoreIndex(args) => {
                    msg!("Instruction: Set Store Index");
                    process_set_store_index(program_id, accounts, args)
                }
                MetaplexInstruction::SetAuctionCache => {
                    msg!("Instruction: Set Auction Cache");
                    process_set_auction_cache(program_id, accounts)
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