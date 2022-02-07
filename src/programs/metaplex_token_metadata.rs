use core::num::flt2dec::Sign;
use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use mpl_token_metadata::deprecated_instruction::SetReservationListArgs;
use serde::Serialize;
use mpl_token_metadata::instruction::MetadataInstruction;
use sha3::digest::Update;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};
use crate::MetaplexTokenMetadataDatum::CreateMetadataAccount;

pub const PROGRAM_ADDRESS: &str = "auctxRXPeJoc4817jDhf4HbjnhEcr1cCXenosMhK5R8";

pub const METAPLEX_TOKEN_METADATA_CREATED_METADATA_TABLE: &str = "metaplex_token_metadata_created_metadatas";
pub const METAPLEX_TOKEN_METADATA_CREATOR_TABLE: &str = "metaplex_token_metadata_creators";
pub const METAPLEX_CLAIMED_BID_TABLE: &str = "metaplex_claimed_bids";
pub const METAPLEX_ENDED_AUCTION_TABLE: &str = "metaplex_ended_auctions";
pub const METAPLEX_STARTED_AUCTION_TABLE: &str = "metaplex_started_auctions";
pub const METAPLEX_SET_AUTHORITY_TABLE: &str = "metaplex_set_authority";

lazy_static! {
    pub static ref METAPLEX_TOKEN_METADATA_CREATED_METADATA_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_created_metadata",
        "fields": [
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "name", "type": "string"},
            {"name": "symbol", "type": "string"},
            {"name": "uri", "type": "string"},
            {"name": "seller_fee_bips", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CREATOR_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_creator",
        "fields": [
            {"name": "token_metadata", "type": "string"},
            {"name": "address", "type": "string"},
            {"name": "verified", "type": "boolean"},
            {"name": "share", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CLAIMED_BID_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_claimed_bid",
        "fields": [
            {"name": "auction", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "bidder_pot_token_account", "type": "string"},
            {"name": "bidder_pot_pda", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "auction_mint", "type": "string"},
            {"name": "token_program", "type": "string"},
            {"name": "auction_extended", "type": "string"},
            {"name": "resource", "type": "string"},
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
            {"name": "auction", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "revealed_price", "type": ["null", "long"]},
            {"name": "revealed_salt", "type": ["null", "long"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_STARTED_AUCTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_started_auction",
        "fields": [
            {"name": "creator", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_SET_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_authority",
        "fields": [
            {"name": "auction", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "new_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_PLACED_BIDS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_placed_bid",
        "fields": [
            {"name": "auction", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "bidder_paying_account", "type": "string"},
            {"name": "pot", "type": "string"},
            {"name": "pot_spl", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "token_mint", "type": "string"},
            {"name": "transfer_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "resource", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CREATED_AUCTIONS_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_created_auction_v2",
        "fields": [
            {"name": "auction", "type": "string"},
            {"name": "auction_extended", "type": "string"},
            {"name": "winners", "type": "long"},
            {"name": "end_auction_at", "type": ["null", "long"]},
            {"name": "end_auction_gap", "type": ["null", "long"]},
            {"name": "token_mint", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "price_floor", "type": "int"},
            {"name": "tick_size", "type": ["null", "long"]},
            {"name": "gap_tick_size_percentage", "type": ["null", "int"]},
            {"name": "instant_sale_price", "type": ["null", "long"]},
            {"name": "name", "type": ["null", "string"]},
            {"name": "creator", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum MetaplexTokenMetadataDatum {
    CreateMetadataAccount(CreatedMetadata),
    UpdateMetadataAccount(UpdatedMetadata),
    DeprecatedCreateMasterEdition(CreatedMasterEdition),
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(MintNewEditionFromMasterEditionViaPrintingToken),
    UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaToken),
    DeprecatedSetReservationList(SetReservationList),
    DeprecatedCreateReservationList(CreateReservationList),
    SignMetadata(SignedMetadata),
    DeprecatedMintPrintingTokensViaToken(MintPrintingTokensViaToken),
    DeprecatedMintPrintingTokens(MintPrintingTokens),
    CreateMasterEdition(CreatedMasterEdition),
    MintNewEditionFromMasterEditionViaToken(MintedNewEditionFromMasterEditionViaToken),
    ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2),
    MintNewEditionFromMasterEditionViaVaultProxy(MintedNewEditionFromMasterEditionViaVaultProxy),
    PuffMetadata(PuffedMetadata),
    UpdateMetadataAccountV2(UpdatedMetadataAccountV2),
    CreateMetadataAccountV2,
    CreateMasterEditionV3,
    VerifyCollection,
    Utilize,
    ApproveUseAuthority,
    RevokeUseAuthority,
    UnverifyCollection,
    ApproveCollectionAuthority,
    RevokeCollectionAuthority,
    SetAndVerifyCollection,
    CreatorData(Creator)
}

/// Struct tables
#[derive(Serialize)]
pub struct CreatedMetadata {
    pub metadata: String,
    pub mint: String,
    pub mint_authority: String,
    pub payer: String,
    pub update_authority: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_bips: i32,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Creator {
    pub token_metadata: String,
    pub address: String,
    pub verified: bool,
    pub share: i16,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct UpdatedMetadata {
    pub metadata: String,
    pub update_authority: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub seller_fee_bips: Option<i32>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintedNewEditionFromMasterEditionViaPrintingToken {
    pub metadata: String,
    pub new_edition: String,
    pub master_record_edition: String,
    pub mint: String,
    pub mint_authority: String,
    pub master_record_printing_mint: String,
    pub edition_pda_mark_creation: String,
    pub burn_authority: String,
    pub payer: String,
    pub update_authority: String,
    pub master_record_metadata: String,
    pub reservation_list: Option<String>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct UpdatePrimarySaleHappenedViaToken {
    pub metadata: String,
    pub owner: String,
    /// Account containing tokens from the metadata's mint
    pub metadata_mint_tokens_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Reservation {
    pub address: String,
    pub spots_remaining: i64,
    pub total_spots: i64,
}

#[derive(Serialize)]
pub struct SetReservationList {
    pub master_edition: String,
    pub reservation_list: String,
    pub reservation_list_resource: String,
    pub total_reservation_spots: Option<i64>,
    pub offset: i64,
    pub total_spot_offset: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreateReservationList {
    pub pda: String,
    pub payer: String,
    pub update_authority: String,
    pub master_edition: String,
    pub reservation_list_resource: String,
    pub metadata: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SignedMetadata {
    pub metadata: String,
    pub creator: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintPrintingTokensViaToken {
    pub destination: String,
    pub one_time_auth_token_account: String,
    pub one_time_auth_mint: String,
    pub printing_mint: String,
    pub burn_authority: String,
    pub metadata: String,
    pub master_edition: String,
    pub supply: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintPrintingTokens {
    pub destination: String,
    pub printing_mint: String,
    pub update_authority: String,
    pub metadata: String,
    pub master_edition: String,
    pub supply: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreatedMasterEdition {
    /// Unallocated edition V1 account with address as pda of ['metadata', program id, mint, 'edition']
    pub account: String,
    pub metadata_mint: String,
    /// Printing mint - A mint you control that can mint tokens that can be exchanged for limited editions of your
    /// master edition via the MintNewEditionFromMasterEditionViaToken endpoint
    pub printing_mint: String,
    pub one_time_authorization_printing_mint: String,
    pub update_authority: String,
    pub printing_mint_authority: String,
    pub metadata_mint_authority: String,
    pub metadata: String,
    pub payer: String,
    pub one_time_authorization_printing_mint_authority: String,
    /// If set, means that no more than this number of editions can ever be minted. This is immutable.
    pub max_supply: Option<i64>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintedNewEditionFromMasterEditionViaToken {
    pub metadata: String,
    pub new_edition: String,
    pub master_record_edition: String,
    pub mint: String,
    pub edition_pda: String,
    pub mint_authority: String,
    pub payer: String,
    pub owner: String,
    pub master_metadata_mint_token_account: String,
    pub update_authority: String,
    pub edition: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ConvertMasterEditionV1ToV2 {
    pub master_record_edition: String,
    pub one_time_authorization_mint: String,
    pub printing_mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintedNewEditionFromMasterEditionViaVaultProxy {
    pub metadata: String,
    pub new_edition: String,
    pub master_record_edition: String,
    pub mint: String,
    pub edition_pda: String,
    pub mint_authority: String,
    pub payer: String,
    pub vault_authority: String,
    pub safety_deposit_token_store: String,
    pub safety_deposit_box: String,
    pub vault: String,
    pub update_authority_info: String,
    pub master_record_metadata: String,
    pub edition: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct PuffedMetadata {
    pub metadata: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Collection {
    pub verified: bool,
    pub key: String
}

#[derive(Serialize)]
pub struct UpdatedMetadataAccountV2 {
    pub metadata: String,
    pub update_authority: String,
    /// The name of the asset
    pub name: Option<String>,
    /// The symbol for the asset
    pub symbol: Option<String>,
    /// URI pointing to JSON representing the asset
    pub uri: Option<String>,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: Option<u16>,
    pub collection: Option<Vec<Collection>>,
    pub use_method: Option<i16>,
    pub remaining: Option<i64>,
    pub uses_total: Option<i64>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintNewEditionFromMasterEditionViaPrintingToken {
    /// New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata_key: String,
    /// New Edition V1 (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: String,
    /// Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_record_edition: String,
    /// Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_token_mint: String,
    /// Mint authority of new mint
    pub mint_authority: String,
    /// Printing Mint of master record edition
    pub printing_mint_master: String,
    pub printing_mint_token_account: String,
    pub marked_creation_edition_pda: String,
    pub burn_authority: String,
    pub payer: String,
    pub update_authority: String,
    pub master_record_metadata: String,
    pub reservation_list: Option<String>,
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
    let token_metadata_dr = MetadataInstruction::try_from_slice(&instruction.data);

    return match token_metadata_dr {
        Ok(ref bld) => {
            let deserialized_mtm_ix = bld.clone();
            let mut response: Vec<TableData> = Vec::new();
            return match deserialized_mtm_ix {
                MetadataInstruction::CreateMetadataAccount(ref mtm_ix) => {
                    response.push(TableData {
                        schema: (*METAPLEX_TOKEN_METADATA_CREATED_METADATA_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CREATED_METADATA_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            CreateMetadataAccount(CreatedMetadata {
                                metadata: instruction.accounts[0].account.to_string(),
                                mint: instruction.accounts[1].account.to_string(),
                                mint_authority: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[3].account.to_string(),
                                update_authority: instruction.accounts[4].account.to_string(),
                                name: mtm_ix.data.name.to_string(),
                                symbol: mtm_ix.data.symbol.to_string(),
                                uri: mtm_ix.data.uri.to_string(),
                                seller_fee_bips: mtm_ix.data.seller_fee_basis_points as i32,
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    let mut creator_data = Vec::new();

                    if let Some(creators) = &mtm_ix.data.creators {
                        for creator in creators {
                            creator_data.push(TypedDatum::MetaplexTokenMetadata(
                                MetaplexTokenMetadataDatum::CreatorData(Creator {
                                    token_metadata: instruction.accounts[0].account.to_string(),
                                    address: creator.address.to_string(),
                                    verified: creator.verified,
                                    share: creator.share as i16,
                                    timestamp: instruction.timestamp
                                })));
                        }
                    }

                    response.push(TableData {
                        schema: (*METAPLEX_CREATOR_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CREATOR_TABLE.to_string(),
                        data: creator_data
                    });

                    Some(response)
                }
                MetadataInstruction::UpdateMetadataAccount(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*).clone(),
                        table_name: .to_string(),
                        data: vec![TypedDatum::]
                    })
                }
                MetadataInstruction::DeprecatedCreateMasterEdition(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => {
                    Some(response)
                }
                MetadataInstruction::UpdatePrimarySaleHappenedViaToken => {
                    Some(response)
                }
                MetadataInstruction::DeprecatedSetReservationList(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::DeprecatedCreateReservationList => {
                    Some(response)
                }
                MetadataInstruction::SignMetadata => {
                    Some(response)
                }
                MetadataInstruction::DeprecatedMintPrintingTokensViaToken(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::DeprecatedMintPrintingTokens(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::CreateMasterEdition(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::MintNewEditionFromMasterEditionViaToken(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::ConvertMasterEditionV1ToV2 => {
                    Some(response)
                }
                MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::PuffMetadata => {
                    Some(response)
                }
                MetadataInstruction::UpdateMetadataAccountV2(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::CreateMetadataAccountV2(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::CreateMasterEditionV3(ref mtm_idx) => {
                    Some(response)
                }
                MetadataInstruction::VerifyCollection => {
                    Some(response)
                }
                MetadataInstruction::Utilize(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::ApproveUseAuthority(ref mtm_ix) => {
                    Some(response)
                }
                MetadataInstruction::RevokeUseAuthority => {
                    Some(response)
                }
                MetadataInstruction::UnverifyCollection => {
                    Some(response)
                }
                MetadataInstruction::ApproveCollectionAuthority => {
                    Some(response)
                }
                MetadataInstruction::RevokeCollectionAuthority => {
                    Some(response)
                }
                MetadataInstruction::SetAndVerifyCollection => {
                    Some(response)
                }
                MetadataInstruction::FreezeDelegatedAccount => {
                    Some(response)
                }
                MetadataInstruction::ThawDelegatedAccount => {
                    Some(response)
                }
            }
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/metaplex_token_metadata] Attempt to parse instruction from program {} failed due to \
        {}.", instruction.program, err);

            None
        }
    }
}