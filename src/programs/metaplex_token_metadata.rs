use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use serde::Serialize;
use mpl_token_metadata::instruction::MetadataInstruction;
use solana_sdk::precompiles::Verify;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

pub const METAPLEX_TOKEN_METADATA_CREATED_METADATA_TABLE: &str = "metaplex_token_metadata_created_metadatas";
pub const METAPLEX_TOKEN_METADATA_CREATOR_TABLE: &str = "metaplex_token_metadata_creators";
pub const METAPLEX_UPDATE_METADATA_ACCOUNT_TABLE: &str = "metaplex_update_metadata_accounts";
pub const METAPLEX_DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_VIA_EDITION_PRINTING_TOKEN_TABLE: &str = "metaplex_deprecated_mint_new_edition_from_master_edition_via_printing_token";
pub const METAPLEX_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_TABLE: &str = "metaplex_minted_from_master_editions";
pub const METAPLEX_UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_TABLE: &str = "metaplex_update_primary_sale_happened_via_tokens";
pub const METAPLEX_TOKEN_METADATA_SET_RESERVATION_LIST_TABLE: &str = "metaplex_token_metadata_set_reservation_lists";
pub const METAPLEX_TOKEN_METADATA_CREATE_RESERVATION_LIST_TABLE: &str = "metaplex_token_metadata_create_reservation_lists";
pub const METAPLEX_TOKEN_METADATA_SIGNED_METADATA_TABLE: &str = "metaplex_token_metadata_signed_metadatas";
pub const METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_VIA_TOKEN_TABLE: &str = "metaplex_token_metadata_mint_printing_tokens_via_tokens";
pub const METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_TABLE: &str = "metaplex_token_metadata_mint_printing_tokens";
pub const METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_TABLE: &str = "metaplex_deprecated_create_master_editions";
pub const METAPLEX_CREATE_MASTER_EDITION_V2_TABLE: &str = "metaplex_deprecated_create_master_edition_v2s";
pub const METAPLEX_TOKEN_METADATA_CONVERTED_MASTER_EDITION_V1_TO_V2_TABLE: &str = "metaplex_token_metadata_converted_master_edition_v1_to_v2s";
pub const METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_TABLE: &str = "metaplex_token_metadata_minted_new_edition_from_master_edition_via_vault_proxies";
pub const METAPLEX_TOKEN_METADATA_PUFFED_METADATA_TABLE: &str = "metaplex_token_metadata_puffed_metadatas";
pub const METAPLEX_TOKEN_METADATA_UPDATED_METADATA_ACCOUNT_V2_TABLE: &str = "metaplex_token_metadata_updated_metadata_account_v2s";
pub const METAPLEX_TOKEN_METADATA_CREATED_METADATA_ACCOUNT_V2_TABLE: &str = "metaplex_token_metadata_created_metadata_account_v2s";
pub const METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_TABLE: &str = "metaplex_token_metadata_minted_new_edition_from_master_edition_via_tokens";
pub const METAPLEX_TOKEN_METADATA_CREATE_MASTER_EDITION_V3_TABLE: &str = "metaplex_token_metadata_create_master_edition_v3s";
pub const METAPLEX_TOKEN_METADATA_VERIFIED_COLLECTION_TABLE: &str = "metaplex_token_metadata_verified_collections";
pub const METAPLEX_TOKEN_METADATA_UTILIZE_TABLE: &str = "metaplex_token_metadata_utilizes";
pub const METAPLEX_TOKEN_METADATA_APPROVED_USE_AUTHORITY_TABLE: &str = "metaplex_token_metadata_approved_use_authorities";
pub const METAPLEX_TOKEN_METADATA_REVOKED_USE_AUTHORITY_TABLE: &str = "metaplex_token_metadata_revoked_use_authorities";
pub const METAPLEX_TOKEN_METADATA_UNVERFIED_COLLECTION_TABLE: &str = "metaplex_token_metadata_unverified_collections";
pub const METAPLEX_TOKEN_METADATA_APPROVED_COLLECION_AUTHORITY_TABLE: &str = "metaplex_token_metadata_approved_collection_authorities";
pub const METAPLEX_TOKEN_METADATA_REVOKED_COLLECTION_AUTHORITY_TABLE: &str = "metaplex_token_metadata_revoked_collection_authorities";
pub const METAPLEX_TOKEN_METADATA_SET_AND_VERIFY_COLLECTION_TABLE: &str = "metaplex_token_metadata_set_and_verify_collections";
pub const METAPLEX_TOKEN_METADATA_FROZEN_DELEGATED_ACCOUNT_TABLE: &str = "metaplex_token_metadata_frozen_delegated_accounts";
pub const METAPLEX_TOKEN_METADATA_THAW_DELEGATED_ACCOUNT_TABLE: &str = "metaplex_token_metadata_thaw_delgated_accounts";

lazy_static! {
    pub static ref METAPLEX_TOKEN_METADATA_CREATED_METADATA_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_created_metadata",
        "fields": [
            {"name": "tx_hash", "type": "string"},
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
            {"name": "tx_hash", "type": "string"},
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
    pub static ref METAPLEX_UPDATE_METADATA_ACCOUNT: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_update_metadata_account",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "name", "type": ["null", "string"]},
            {"name": "symbol", "type": ["null", "string"]},
            {"name": "uri", "type": ["null", "string"]},
            {"name": "seller_fee_bips", "type": ["null", "int"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex-deprecated_mint_new_edition_from_master_edition_via_printing_token,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_record_edition", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "master_record_printing_mint", "type": "string"},
            {"name": "edition_pda_mark_creation", "type": "string"},
            {"name": "burn_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "master_record_metadata", "type": "string"},
            {"name": "reservation_list", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_update_primary_sale_happened_via_token",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "owner", "type": "string"},
            {"name": "metadata_mint_tokens_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_SET_RESERVATION_LIST_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_reservation_list",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "reservation_list", "type": "string"},
            {"name": "reservation_list_resource", "type": "string"},
            {"name": "reservations", "type": "array", "items": {
                "type": "record",
                "name": "reservation",
                "fields": [
                    {"name": "address", "type": "string"},
                    {"name": "spots_remaining", "type": "long"},
                    {"name": "total_spots", "type": "long"}
                ]
            }}
            {"name": "total_reservation_spots", "type": ["null","long"]},
            {"name": "offset", "type": "long"},
            {"name": "total_spot_offset", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_CREATE_RESERVATION_LIST_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_create_reservation_list",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "pda", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "reservation_list_resource", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_SIGNED_METADATA_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_signed_metadata",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "creator", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_VIA_TOKEN_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_mint_printing_tokens_via_token",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "one_time_auth_token_account", "type": "string"},
            {"name": "one_time_auth_mint", "type": "string"},
            {"name": "printing_mint", "type": "string"},
            {"name": "burn_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "supply", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_mint_printing_tokens",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "printing_mint", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "master_edition", "type": "string"},
            {"name": "supply", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex-deprecated_create_master_edition,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "metadata_mint", "type": "string"},
            {"name": "printing_mint", "type": "string"},
            {"name": "one_time_authorization_printing_mint", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "printing_mint_authority", "type": "string"},
            {"name": "metadata_mint_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "one_time_authorization_printing_mint_authority", "type": "string"},
            {"name": "max_supply", "type": ["null", "int"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_minted_new_edition_from_master_edition_via_token,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_record_edition", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "edition_pda", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "owner", "type": "string"},
            {"name": "master_metadata_mint_token_account", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "master_record_metadata", "type": "string"},
            {"name": "edition", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_CONVERTED_MASTER_EDITION_V1_TO_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_converted_master_edition_v1_to_v2,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "master_record_edition", "type": "string"},
            {"name": "one_time_authorization_mint", "type": "string"},
            {"name": "printing_mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_minted_new_edition_from_master_edition_via_vault_proxy,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_record_edition", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "master_record_printing_mint", "type": "string"},
            {"name": "edition_pda_mark_creation", "type": "string"},
            {"name": "burn_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "master_record_metadata", "type": "string"},
            {"name": "reservation_list", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_TOKEN_METADATA_PUFFED_METADATA_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_token_metadata_puffed_metadata",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

    pub static ref METAPLEX_CREATE_MASTER_EDITION_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_create_master_edition_v2,
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "edition_account", "type": "string"},
            {"name": "metadata_mint", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "max_supply", "type": ["null", "long"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_UPDATED_METADATA_ACCOUNT_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_created_metadata_account_v2",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "name", "type": ["null", "string"]},
            {"name": "symbol", "type": ["null", "string"]},
            {"name": "uri", "type": ["null", "string"]},
            {"name": "seller_fee_basis_points", "type": ["null", "int"]},
            {"name": "is_verified_collection", "type": ["null", "bool"]},
            {"name": "collection_key", "type": ["null", "string"]},
            {"name": "use_method", "type": ["null", "int"]},
            {"name": "remaining", "type": ["null", "int"]},
            {"name": "uses_total", "type": ["null", "int"]},
            {"name": "primary_sale_happened", "type": ["null", "bool"]},
            {"name": "is_mutable", "type": ["null", "bool"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_CREATED_METADATA_ACCOUNT_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_created_metadata_account_v2",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "name", "type": "string"},
            {"name": "symbol", "type": "string"},
            {"name": "uri", "type": "string"},
            {"name": "seller_fee_basis_points", "type": "int"},
            {"name": "is_verified_collection", "type": ["null", "bool"]},
            {"name": "collection_key", "type": ["null", "string"]},
            {"name": "use_method", "type": ["null", "int"]},
            {"name": "remaining", "type": ["null", "int"]},
            {"name": "uses_total", "type": ["null", "int"]},
            {"name": "is_mutable", "type": "bool"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_minted_new_edition_from_master_edition_via_printing_token",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "new_edition", "type": "string"},
            {"name": "master_record_edition", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "master_record_printing_mint", "type": "string"},
            {"name": "edition_pda_mark_creation", "type": "string"},
            {"name": "burn_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "master_record_metadata", "type": "string"},
            {"name": "reservation_list", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_CREATE_MASTER_EDITION_V3_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_create_master_edition_v3",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "metadata_mint", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "max_supply", "type": ["null", "int"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();


    pub static ref METAPLEX_TOKEN_METADATA_VERIFIED_COLLECTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_verified_collection",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "collection_mint", "type": "string"},
            {"name": "collection_metadata", "type": "string"},
            {"name": "edition_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_UTILIZE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_utilize",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "token_account", "type": "string"},
            {"name": "metadata_mint", "type": "string"},
            {"name": "use_authority", "type": "string"},
            {"name": "owner", "type": "string"},
            {"name": "number_of_uses", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();
    pub static ref METAPLEX_APPROVED_USE_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_approved_use_authority",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "authority_record_pda", "type": "string"},
            {"name": "owned_token_account", "type": "string"},
            {"name": "owner", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "use_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "burner", "type": "string"},
            {"name": "number_of_uses", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_REVOKED_USE_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_revoked_use_authority",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "authority_record_pda", "type": "string"},
            {"name": "owned_token_account", "type": "string"},
            {"name": "owner", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "use_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_UNVERFIED_COLLECTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_unverified_collection",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "collection_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "collection_metadata", "type": "string"},
            {"name": "edition_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_APPROVED_COLLECION_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_approved_collection_authority",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "collection_authority_pda", "type": "string"},
            {"name": "update_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "collection_authority", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_REVOKED_COLLECTION_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_revoked_collection_authority",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "use_authority_pda", "type": "string"},
            {"name": "owned_token_account", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_SET_AND_VERIFY_COLLECTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_and_verify_collection",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "collection_update_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "update_authority_of_collection", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "collection_metadata", "type": "string"},
            {"name": "edition_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_FROZEN_DELEGATED_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_frozen_delegated_account",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "delegate", "type": "string"},
            {"name": "frozen", "type": "string"},
            {"name": "edition", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
        "#
    )
    .unwrap();

    pub static ref METAPLEX_TOKEN_METADATA_THAW_DELEGATED_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_thaw_delegated_account",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "delegate", "type": "string"},
            {"name": "thaw", "type": "string"},
            {"name": "edition", "type": "string"},
            {"name": "mint", "type": "string"},
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
    /// V1
    CreateMasterEdition(CreatedMasterEdition),
    /// V2
    CreateMasterEditionV2(CreatedMasterEditionV2),
    MintNewEditionFromMasterEditionViaToken(MintedNewEditionFromMasterEditionViaToken),
    ConvertMasterEditionV1ToV2(ConvertedMasterEditionV1ToV2),
    MintNewEditionFromMasterEditionViaVaultProxy(MintedNewEditionFromMasterEditionViaVaultProxy),
    PuffMetadata(PuffedMetadata),
    UpdateMetadataAccountV2(UpdatedMetadataAccountV2),
    CreateMetadataAccountV2(CreatedMetadataAccountV2),
    CreateMasterEditionV3(CreateMasterEditionV3),
    VerifyCollection(VerifiedCollection),
    Utilize(Utilize),
    ApproveUseAuthority(ApprovedUseAuthority),
    RevokeUseAuthority(RevokedUseAuthority),
    UnverifyCollection(UnverifiedCollection),
    ApproveCollectionAuthority(ApprovedCollectionAuthority),
    RevokeCollectionAuthority(RevokedCollectionAuthority),
    SetAndVerifyCollection(SetAndVerifyCollection),
    FreezeDelegatedAccount(FrozenDelegatedAccount),
    ThawDelegatedAccount(ThawDelegatedAccount),
    CreatorData(Creator)
}

/// Struct tables
#[derive(Serialize)]
pub struct CreatedMetadata {
    pub tx_hash: String,
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
    pub tx_hash: String,
    pub token_metadata: String,
    pub address: String,
    pub verified: bool,
    pub share: i16,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct UpdatedMetadata {
    pub tx_hash: String,
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
    pub tx_hash: String,
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
    pub tx_hash: String,
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
    pub total_spots: i64
}

#[derive(Serialize)]
pub struct SetReservationList {
    pub tx_hash: String,
    pub master_edition: String,
    pub reservation_list: String,
    pub reservation_list_resource: String,
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: Option<i64>,
    pub offset: i64,
    pub total_spot_offset: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreateReservationList {
    pub tx_hash: String,
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
    pub tx_hash: String,
    pub metadata: String,
    pub creator: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintPrintingTokensViaToken {
    pub tx_hash: String,
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
    pub tx_hash: String,
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
    pub tx_hash: String,
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
pub struct CreatedMasterEditionV2 {
    pub tx_hash: String,
    pub edition_account: String,
    pub metadata_mint: String,
    pub update_authority: String,
    pub mint_authority: String,
    pub payer: String,
    pub metadata: String,
    pub max_supply: Option<i64>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintedNewEditionFromMasterEditionViaToken {
    pub tx_hash: String,
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
    pub master_record_metadata: String,
    pub edition: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ConvertedMasterEditionV1ToV2 {
    pub tx_hash: String,
    pub master_record_edition: String,
    pub one_time_authorization_mint: String,
    pub printing_mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintedNewEditionFromMasterEditionViaVaultProxy {
    pub tx_hash: String,
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
    pub tx_hash: String,
    pub metadata: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Collection {
    pub tx_hash: String,
    pub verified: bool,
    pub key: String
}

#[derive(Serialize)]
pub struct UpdatedMetadataAccountV2 {
    pub tx_hash: String,
    pub metadata: String,
    pub update_authority: String,
    /// The name of the asset
    pub name: Option<String>,
    /// The symbol for the asset
    pub symbol: Option<String>,
    /// URI pointing to JSON representing the asset
    pub uri: Option<String>,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: Option<i32>,
    pub is_verified_collection: Option<bool>,
    pub collection_key: Option<String>,
    pub use_method: Option<i16>,
    pub remaining: Option<i64>,
    pub uses_total: Option<i64>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreatedMetadataAccountV2 {
    pub tx_hash: String,
    pub metadata: String,
    pub mint: String,
    pub mint_authority: String,
    pub payer: String,
    pub update_authority: String,
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: i32,
    pub is_verified_collection: Option<bool>,
    pub collection_key: Option<String>,
    pub use_method: Option<i16>,
    pub remaining: Option<i64>,
    pub uses_total: Option<i64>,
    pub is_mutable: bool,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintNewEditionFromMasterEditionViaPrintingToken {
    pub tx_hash: String,
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

#[derive(Serialize)]
pub struct CreateMasterEditionV3 {
    pub tx_hash: String,
    pub account: String,
    pub metadata_mint: String,
    pub update_authority: String,
    pub mint_authority: String,
    pub payer: String,
    pub metadata: String,
    pub max_supply: Option<i64>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct VerifiedCollection {
    pub tx_hash: String,
    pub metadata: String,
    pub update_authority: String,
    pub payer: String,
    pub collection_mint: String,
    pub collection_metadata: String,
    /// MasterEdition2 Account of the Collection Token
    pub edition_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Utilize {
    pub tx_hash: String,
    pub metadata: String,
    pub token_account: String,
    pub metadata_mint: String,
    pub use_authority: String,
    pub owner: String,
    pub number_of_uses: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ApprovedUseAuthority {
    pub tx_hash: String,
    pub authority_record_pda: String,
    pub owned_token_account: String,
    pub owner: String,
    pub payer: String,
    pub use_authority: String,
    pub metadata: String,
    pub mint: String,
    pub burner: String,
    pub number_of_uses: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct RevokedUseAuthority {
    pub tx_hash: String,
    pub authority_record_pda: String,
    pub owned_token_account: String,
    pub owner: String,
    pub payer: String,
    pub use_authority: String,
    pub metadata: String,
    pub mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct UnverifiedCollection {
    pub tx_hash: String,
    pub metadata: String,
    pub collection_authority: String,
    pub payer: String,
    pub mint: String,
    pub collection_metadata: String,
    pub edition_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ApprovedCollectionAuthority {
    pub tx_hash: String,
    pub collection_authority_pda: String,
    pub update_authority: String,
    pub payer: String,
    pub collection_authority: String,
    pub metadata: String,
    pub mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct RevokedCollectionAuthority {
    pub tx_hash: String,
    pub use_authority_pda: String,
    pub owned_token_account: String,
    pub metadata: String,
    pub mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SetAndVerifyCollection {
    pub tx_hash: String,
    pub metadata: String,
    pub collection_update_authority: String,
    pub payer: String,
    pub update_authority_of_collection: String,
    pub mint: String,
    pub collection_metadata: String,
    pub edition_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct FrozenDelegatedAccount {
    pub tx_hash: String,
    pub delegate: String,
    pub frozen: String,
    pub edition: String,
    pub mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ThawDelegatedAccount {
    pub tx_hash: String,
    pub delegate: String,
    pub thaw: String,
    pub edition: String,
    pub mint: String,
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
                            MetaplexTokenMetadataDatum::CreateMetadataAccount(CreatedMetadata {
                                tx_hash: instruction.transaction_hash.to_string(),
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
                                    tx_hash: instruction.transaction_hash.to_string(),
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
                        schema: (*METAPLEX_UPDATE_METADATA_ACCOUNT).clone(),
                        table_name: METAPLEX_UPDATE_METADATA_ACCOUNT_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::UpdateMetadataAccount(UpdatedMetadata {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                update_authority: instruction.accounts[1].account.to_string(),
                                name: if let Some(data) = &mtm_ix.data {
                                    Some(data.name.to_string())
                                } else {
                                    None
                                },
                                symbol: if let Some(data) = &mtm_ix.data {
                                    Some(data.symbol.to_string())
                                } else {
                                    None
                                },
                                uri: if let Some(data) = &mtm_ix.data {
                                    Some(data.uri.to_string())
                                } else {
                                    None
                                },
                                seller_fee_bips: if let Some(data) = &mtm_ix.data {
                                    Some(data.seller_fee_basis_points as i32)
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });
                    Some(response)
                }
                MetadataInstruction::DeprecatedCreateMasterEdition(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_SCHEMA).clone(),
                        table_name: METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedCreateMasterEdition(CreatedMasterEdition {
                                tx_hash: instruction.transaction_hash.to_string(),
                                account: instruction.accounts[0].account.to_string(),
                                metadata_mint: instruction.accounts[1].account.to_string(),
                                printing_mint: instruction.accounts[2].account.to_string(),
                                one_time_authorization_printing_mint: instruction.accounts[3].account.to_string(),
                                update_authority: instruction.accounts[4].account.to_string(),
                                printing_mint_authority: instruction.accounts[5].account.to_string(),
                                metadata_mint_authority: instruction.accounts[6].account.to_string(),
                                metadata: instruction.accounts[7].account.to_string(),
                                payer: instruction.accounts[8].account.to_string(),
                                one_time_authorization_printing_mint_authority: instruction.accounts[9].account.to_string(),
                                max_supply: if let Some(ms) = mtm_ix.max_supply {
                                    Some(ms as i64)
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });
                    Some(response)
                }
                MetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => {
                    response.push(TableData{
                        schema: (*METAPLEX_DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN).clone(),
                        table_name: METAPLEX_DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_VIA_EDITION_PRINTING_TOKEN_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(MintNewEditionFromMasterEditionViaPrintingToken {
                                tx_hash: instruction.transaction_hash.to_string(),
                                new_metadata_key: instruction.accounts[0].account.to_string(),
                                new_edition: instruction.accounts[1].account.to_string(),
                                master_record_edition: instruction.accounts[2].account.to_string(),
                                new_token_mint: instruction.accounts[3].account.to_string(),
                                mint_authority: instruction.accounts[4].account.to_string(),
                                printing_mint_master: instruction.accounts[5].account.to_string(),
                                printing_mint_token_account: instruction.accounts[6].account.to_string(),
                                marked_creation_edition_pda: instruction.accounts[7].account.to_string(),
                                burn_authority: instruction.accounts[8].account.to_string(),
                                payer: instruction.accounts[9].account.to_string(),
                                update_authority: instruction.accounts[10].account.to_string(),
                                master_record_metadata: instruction.accounts[10].account.to_string(),
                                reservation_list: if instruction.accounts.len() > 15 {
                                    Some(instruction.accounts[11].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::UpdatePrimarySaleHappenedViaToken => {
                    response.push(TableData{
                        schema: (*METAPLEX_UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_SCHEMA).clone(),
                        table_name: METAPLEX_UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaToken {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                owner: instruction.accounts[1].account.to_string(),
                                metadata_mint_tokens_account: instruction.accounts[2].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::DeprecatedSetReservationList(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_SET_RESERVATION_LIST_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_SET_RESERVATION_LIST_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedSetReservationList(SetReservationList {
                                tx_hash: instruction.transaction_hash.to_string(),
                                master_edition: instruction.accounts[0].account.to_string(),
                                reservation_list: instruction.accounts[1].account.to_string(),
                                reservation_list_resource: instruction.accounts[2].account.to_string(),
                                reservations: mtm_ix.reservations.as_slice().into_iter()
                                    .map(|rs| {
                                        Reservation {
                                            address: rs.address.to_string(),
                                            spots_remaining: rs.spots_remaining as i64,
                                            total_spots: rs.total_spots as i64
                                        }
                                    })
                                    .collect(),
                                total_reservation_spots: if let Some(trs) = &mtm_ix.total_reservation_spots {
                                    Some(trs.clone() as i64)
                                } else {
                                    None
                                },
                                offset: mtm_ix.offset as i64,
                                total_spot_offset: mtm_ix.total_spot_offset as i64,
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::DeprecatedCreateReservationList => {
                    response.push(TableData{
                        schema: (*METAPLEX_CREATE_RESERVATION_LIST_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CREATE_RESERVATION_LIST_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedCreateReservationList(
                                CreateReservationList {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    pda: instruction.accounts[0].account.to_string(),
                                    payer: instruction.accounts[1].account.to_string(),
                                    update_authority: instruction.accounts[2].account.to_string(),
                                    master_edition: instruction.accounts[3].account.to_string(),
                                    reservation_list_resource: instruction.accounts[4].account.to_string(),
                                    metadata: instruction.accounts[5].account.to_string(),
                                    timestamp: instruction.timestamp
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::SignMetadata => {
                    response.push(TableData{
                        schema: (*METAPLEX_SIGNED_METADATA_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_SIGNED_METADATA_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::SignMetadata(
                                SignedMetadata {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    metadata: instruction.accounts[0].account.to_string(),
                                    creator: instruction.accounts[1].account.to_string(),
                                    timestamp: instruction.timestamp
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::DeprecatedMintPrintingTokensViaToken(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedMintPrintingTokensViaToken(
                                MintPrintingTokensViaToken {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    destination: instruction.accounts[0].account.to_string(),
                                    one_time_auth_token_account: instruction.accounts[1].account.to_string(),
                                    one_time_auth_mint: instruction.accounts[2].account.to_string(),
                                    printing_mint: instruction.accounts[3].account.to_string(),
                                    burn_authority: instruction.accounts[4].account.to_string(),
                                    metadata: instruction.accounts[5].account.to_string(),
                                    master_edition: instruction.accounts[6].account.to_string(),
                                    supply: mtm_ix.supply as i64,
                                    timestamp: instruction.timestamp
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::DeprecatedMintPrintingTokens(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_MINT_PRINTING_TOKENS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::DeprecatedMintPrintingTokens(
                                MintPrintingTokens {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    destination: instruction.accounts[0].account.to_string(),
                                    printing_mint: instruction.accounts[1].account.to_string(),
                                    update_authority: instruction.accounts[2].account.to_string(),
                                    metadata: instruction.accounts[3].account.to_string(),
                                    master_edition: instruction.accounts[4].account.to_string(),
                                    supply: mtm_ix.supply as i64,
                                    timestamp: instruction.timestamp
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::CreateMasterEdition(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_CREATE_MASTER_EDITION_V2_SCHEMA).clone(),
                        table_name: METAPLEX_CREATE_MASTER_EDITION_V2_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::CreateMasterEditionV2(
                                CreatedMasterEditionV2 {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    edition_account: instruction.accounts[0].account.to_string(),
                                    metadata_mint: instruction.accounts[1].account.to_string(),
                                    update_authority: instruction.accounts[2].account.to_string(),
                                    mint_authority: instruction.accounts[3].account.to_string(),
                                    payer: instruction.accounts[4].account.to_string(),
                                    metadata: instruction.accounts[5].account.to_string(),
                                    max_supply: if let Some(ms) = &mtm_ix.max_supply {
                                        Some(ms.clone() as i64)
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::MintNewEditionFromMasterEditionViaToken(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::MintNewEditionFromMasterEditionViaToken(
                                MintedNewEditionFromMasterEditionViaToken {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    metadata: instruction.accounts[0].account.to_string(),
                                    new_edition: instruction.accounts[1].account.to_string(),
                                    master_record_edition: instruction.accounts[2].account.to_string(),
                                    mint: instruction.accounts[3].account.to_string(),
                                    edition_pda: instruction.accounts[4].account.to_string(),
                                    mint_authority: instruction.accounts[5].account.to_string(),
                                    payer: instruction.accounts[6].account.to_string(),
                                    owner: instruction.accounts[7].account.to_string(),
                                    master_metadata_mint_token_account: instruction.accounts[8].account.to_string(),
                                    update_authority: instruction.accounts[9].account.to_string(),
                                    master_record_metadata: instruction.accounts[10].account.to_string(),
                                    edition: mtm_ix.edition as i64,
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::ConvertMasterEditionV1ToV2 => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_CONVERTED_MASTER_EDITION_V1_TO_V2_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CONVERTED_MASTER_EDITION_V1_TO_V2_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::ConvertMasterEditionV1ToV2(
                                ConvertedMasterEditionV1ToV2 {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    master_record_edition: instruction.accounts[0].account.to_string(),
                                    one_time_authorization_mint: instruction.accounts[1].account.to_string(),
                                    printing_mint: instruction.accounts[2].account.to_string(),
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_MINTED_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::MintNewEditionFromMasterEditionViaVaultProxy(
                                MintedNewEditionFromMasterEditionViaVaultProxy {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    metadata: instruction.accounts[0].account.to_string(),
                                    new_edition: instruction.accounts[1].account.to_string(),
                                    master_record_edition: instruction.accounts[2].account.to_string(),
                                    mint: instruction.accounts[3].account.to_string(),
                                    edition_pda: instruction.accounts[4].account.to_string(),
                                    mint_authority: instruction.accounts[5].account.to_string(),
                                    payer: instruction.accounts[6].account.to_string(),
                                    vault_authority: instruction.accounts[7].account.to_string(),
                                    safety_deposit_token_store: instruction.accounts[8].account.to_string(),
                                    safety_deposit_box: instruction.accounts[9].account.to_string(),
                                    vault: instruction.accounts[10].account.to_string(),
                                    update_authority_info: instruction.accounts[11].account.to_string(),
                                    master_record_metadata: instruction.accounts[12].account.to_string(),
                                    edition: mtm_ix.edition as i64,
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::PuffMetadata => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_PUFFED_METADATA_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_PUFFED_METADATA_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::PuffMetadata(
                                PuffedMetadata {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    metadata: instruction.accounts[0].account.to_string(),
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::UpdateMetadataAccountV2(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_UPDATED_METADATA_ACCOUNT_V2_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_UPDATED_METADATA_ACCOUNT_V2_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::UpdateMetadataAccountV2(
                                UpdatedMetadataAccountV2 {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    metadata: instruction.accounts[0].account.to_string(),
                                    update_authority: instruction.accounts[1].account.to_string(),
                                    name: if let Some(data) = &mtm_ix.data {
                                        Some(data.name.to_string())
                                    } else {
                                        None
                                    },
                                    symbol: if let Some(data) = &mtm_ix.data {
                                        Some(data.symbol.to_string())
                                    } else {
                                        None
                                    },
                                    uri: if let Some(data) = &mtm_ix.data {
                                        Some(data.uri.to_string())
                                    } else {
                                        None
                                    },
                                    seller_fee_basis_points: if let Some(data) = &mtm_ix.data {
                                        Some(data.seller_fee_basis_points.clone() as i32)
                                    } else {
                                        None
                                    },
                                    is_verified_collection: if let Some(data) = &mtm_ix.data {
                                        if let Some(collection) = &data.collection {
                                            Some(collection.verified)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    },
                                    collection_key: if let Some(data) = &mtm_ix.data {
                                        if let Some(collection) = &data.collection {
                                            Some(collection.key.to_string())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    },
                                    use_method: if let Some(data) = &mtm_ix.data {
                                        if let Some(u) = &data.uses {
                                            Some(u.use_method.clone() as i16)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    },
                                    remaining: if let Some(data) = &mtm_ix.data {
                                        if let Some(u) = &data.uses {
                                            Some(u.remaining as i64)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    },
                                    uses_total: if let Some(data) = &mtm_ix.data {
                                        if let Some(u) = &data.uses {
                                            Some(u.total as i64)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    },
                                    primary_sale_happened: if let Some(psh) = &mtm_ix.primary_sale_happened {
                                        Some(psh.clone())
                                    } else {
                                        None
                                    },
                                    is_mutable: if let Some(im) = &mtm_ix.is_mutable {
                                        Some(im.clone())
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp,
                                })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::CreateMetadataAccountV2(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_CREATED_METADATA_ACCOUNT_V2_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CREATED_METADATA_ACCOUNT_V2_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::CreateMetadataAccountV2(CreatedMetadataAccountV2 {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                mint: instruction.accounts[1].account.to_string(),
                                mint_authority: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[3].account.to_string(),
                                update_authority: instruction.accounts[4].account.to_string(),
                                name: mtm_ix.data.name.clone(),
                                symbol: mtm_ix.data.symbol.clone(),
                                uri: mtm_ix.data.uri.clone(),
                                seller_fee_basis_points: mtm_ix.data.seller_fee_basis_points as i32,
                                is_verified_collection: if let Some(collection) = &mtm_ix.data.collection {
                                    Some(collection.verified)
                                } else {
                                    None
                                },
                                collection_key: if let Some(collection) = &mtm_ix.data.collection {
                                    Some(collection.key.to_string())
                                } else {
                                    None
                                },
                                use_method: if let Some(u) = &mtm_ix.data.uses {
                                    Some(u.use_method.clone() as i16)
                                } else {
                                    None
                                },
                                remaining: if let Some(u) = &mtm_ix.data.uses {
                                    Some(u.remaining as i64)
                                } else {
                                    None
                                },
                                uses_total: if let Some(u) = &mtm_ix.data.uses {
                                    Some(u.total as i64)
                                } else {
                                    None
                                },
                                is_mutable: mtm_ix.is_mutable,
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    let mut creator_data = Vec::new();

                    if let Some(creators) = &mtm_ix.data.creators {
                        for creator in creators {
                            creator_data.push(TypedDatum::MetaplexTokenMetadata(
                                MetaplexTokenMetadataDatum::CreatorData(Creator {
                                    tx_hash: instruction.transaction_hash.to_string(),
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
                MetadataInstruction::CreateMasterEditionV3(ref mtm_idx) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_CREATE_MASTER_EDITION_V3_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_CREATE_MASTER_EDITION_V3_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::CreateMasterEditionV3(CreateMasterEditionV3 {
                                tx_hash: instruction.transaction_hash.to_string(),
                                account: instruction.accounts[0].account.to_string(),
                                metadata_mint: instruction.accounts[1].account.to_string(),
                                update_authority: instruction.accounts[2].account.to_string(),
                                mint_authority: instruction.accounts[3].account.to_string(),
                                payer: instruction.accounts[4].account.to_string(),
                                metadata: instruction.accounts[5].account.to_string(),
                                max_supply: if let Some(ms) = &mtm_idx.max_supply {
                                    Some(ms.clone() as i64)
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::VerifyCollection => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_VERIFIED_COLLECTION_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_VERIFIED_COLLECTION_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::VerifyCollection(VerifiedCollection {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                update_authority: instruction.accounts[1].account.to_string(),
                                payer: instruction.accounts[2].account.to_string(),
                                collection_mint: instruction.accounts[3].account.to_string(),
                                collection_metadata: instruction.accounts[4].account.to_string(),
                                edition_account: instruction.accounts[5].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::Utilize(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_UTILIZE_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_UTILIZE_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::Utilize(Utilize {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                token_account: instruction.accounts[1].account.to_string(),
                                metadata_mint: instruction.accounts[2].account.to_string(),
                                use_authority: instruction.accounts[3].account.to_string(),
                                owner: instruction.accounts[4].account.to_string(),
                                number_of_uses: mtm_ix.number_of_uses as i64,
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::ApproveUseAuthority(ref mtm_ix) => {
                    response.push(TableData{
                        schema: (*METAPLEX_APPROVED_USE_AUTHORITY_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_APPROVED_USE_AUTHORITY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::ApproveUseAuthority(ApprovedUseAuthority {
                                tx_hash: instruction.transaction_hash.to_string(),
                                authority_record_pda: instruction.accounts[0].account.to_string(),
                                owned_token_account: instruction.accounts[1].account.to_string(),
                                owner: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[3].account.to_string(),
                                use_authority: instruction.accounts[4].account.to_string(),
                                metadata: instruction.accounts[5].account.to_string(),
                                mint: instruction.accounts[6].account.to_string(),
                                burner: instruction.accounts[7].account.to_string(),
                                number_of_uses: mtm_ix.number_of_uses as i64,
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::RevokeUseAuthority => {
                    response.push(TableData{
                        schema: (*METAPLEX_REVOKED_USE_AUTHORITY_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_REVOKED_USE_AUTHORITY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::RevokeUseAuthority(RevokedUseAuthority {
                                tx_hash: instruction.transaction_hash.to_string(),
                                authority_record_pda: instruction.accounts[0].account.to_string(),
                                owned_token_account: instruction.accounts[1].account.to_string(),
                                owner: instruction.accounts[2].account.to_string(),
                                payer: instruction.accounts[3].account.to_string(),
                                use_authority: instruction.accounts[4].account.to_string(),
                                metadata: instruction.accounts[5].account.to_string(),
                                mint: instruction.accounts[6].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::UnverifyCollection => {
                    response.push(TableData{
                        schema: (*METAPLEX_UNVERFIED_COLLECTION_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_UNVERFIED_COLLECTION_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::UnverifyCollection(UnverifiedCollection {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                collection_authority: instruction.accounts[1].account.to_string(),
                                payer: instruction.accounts[2].account.to_string(),
                                mint: instruction.accounts[3].account.to_string(),
                                collection_metadata: instruction.accounts[4].account.to_string(),
                                edition_account: instruction.accounts[5].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::ApproveCollectionAuthority => {
                    response.push(TableData{
                        schema: (*METAPLEX_APPROVED_USE_AUTHORITY_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_APPROVED_COLLECION_AUTHORITY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::ApproveCollectionAuthority(ApprovedCollectionAuthority {
                                tx_hash: instruction.transaction_hash.to_string(),
                                collection_authority_pda: instruction.accounts[0].account.to_string(),
                                update_authority: instruction.accounts[1].account.to_string(),
                                payer: instruction.accounts[2].account.to_string(),
                                collection_authority: instruction.accounts[3].account.to_string(),
                                metadata: instruction.accounts[4].account.to_string(),
                                mint: instruction.accounts[5].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::RevokeCollectionAuthority => {
                    response.push(TableData{
                        schema: (*METAPLEX_REVOKED_COLLECTION_AUTHORITY_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_REVOKED_COLLECTION_AUTHORITY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::RevokeCollectionAuthority(RevokedCollectionAuthority {
                                tx_hash: instruction.transaction_hash.to_string(),
                                use_authority_pda: instruction.accounts[0].account.to_string(),
                                owned_token_account: instruction.accounts[1].account.to_string(),
                                metadata: instruction.accounts[2].account.to_string(),
                                mint: instruction.accounts[3].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::SetAndVerifyCollection => {
                    response.push(TableData{
                        schema: (*METAPLEX_SET_AND_VERIFY_COLLECTION_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_SET_AND_VERIFY_COLLECTION_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::SetAndVerifyCollection(SetAndVerifyCollection {
                                tx_hash: instruction.transaction_hash.to_string(),
                                metadata: instruction.accounts[0].account.to_string(),
                                collection_update_authority: instruction.accounts[1].account.to_string(),
                                payer: instruction.accounts[2].account.to_string(),
                                update_authority_of_collection: instruction.accounts[3].account.to_string(),
                                mint: instruction.accounts[4].account.to_string(),
                                collection_metadata: instruction.accounts[5].account.to_string(),
                                edition_account: instruction.accounts[6].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::FreezeDelegatedAccount => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_FROZEN_DELEGATED_ACCOUNT_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_FROZEN_DELEGATED_ACCOUNT_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::FreezeDelegatedAccount(FrozenDelegatedAccount {
                                tx_hash: instruction.transaction_hash.to_string(),
                                delegate: instruction.accounts[0].account.to_string(),
                                frozen: instruction.accounts[1].account.to_string(),
                                edition: instruction.accounts[2].account.to_string(),
                                mint: instruction.accounts[3].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
                }
                MetadataInstruction::ThawDelegatedAccount => {
                    response.push(TableData{
                        schema: (*METAPLEX_TOKEN_METADATA_THAW_DELEGATED_ACCOUNT_SCHEMA).clone(),
                        table_name: METAPLEX_TOKEN_METADATA_THAW_DELEGATED_ACCOUNT_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexTokenMetadata(
                            MetaplexTokenMetadataDatum::ThawDelegatedAccount(ThawDelegatedAccount {
                                tx_hash: instruction.transaction_hash.to_string(),
                                delegate: instruction.accounts[0].account.to_string(),
                                thaw: instruction.accounts[1].account.to_string(),
                                edition: instruction.accounts[2].account.to_string(),
                                mint: instruction.accounts[3].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

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