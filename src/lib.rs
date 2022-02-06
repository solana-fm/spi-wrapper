#[macro_use]
extern crate lazy_static;

pub mod programs;

use avro_rs::{Codec, Writer, Schema};
use serde::{Serialize, Deserialize};
use solana_sdk::instruction::CompiledInstruction;
use serde::de::DeserializeOwned;
use solana_sdk::transaction::Transaction;
use tokio::spawn;
use tracing::info;

use crate::programs::bpf_loader::BpfLoaderDatum;
use crate::programs::bpf_loader_upgradeable::BpfUpgradeableLoaderDatum;
use crate::programs::metaplex::MetaplexMainDatum;
use crate::programs::metaplex_auction::MetaplexAuctionDatum;
use crate::programs::native_associated_token_account::NativeAssociatedTokenAccountDatum;
use crate::programs::native_config::NativeConfigDatum;
use crate::programs::native_loader::NativeLoaderDatum;
use crate::programs::native_secp256k1::NativeSecp256k1Datum;
use crate::programs::native_stake::NativeStakeDatum;
use crate::programs::native_system::NativeSystemDatum;
use crate::programs::native_token::NativeTokenDatum;
use crate::programs::native_token_lending::TokenLendingDatum;
use crate::programs::native_token_swap::NativeTokenSwapDatum;
use crate::programs::native_vote::VoteDatum;
use crate::programs::serum_market::SerumMarketDatum;
use crate::programs::step_token_swap::StepTokenSwapDatum;

#[derive(Clone, Serialize)]
pub struct AccountInstruction {
    // The local unique identifier of the instruction according to the transaction (not based on solana)
    pub tx_instruction_id: i16,
    // The local unique identifier of the instruction type (not based on solana)
    pub transaction_hash: String,
    // If this is an inner instruction, we should depend on this
    pub parent_index: i16,
    // The account pubkey relating to this transaction.
    pub account: String,
    // The index of the account stored in the instruction's slice.
    pub index: i32,
    // The time this was created in terms of the database
    pub timestamp: i64,
}

#[derive(Clone, Serialize)]
pub struct Instruction {
    // The local unique identifier of the instruction according to the transaction (not based on solana)
    pub tx_instruction_id: i16,
    // The transaction this instruction belongs to.
    pub transaction_hash: String,
    // The name of the program invoking this instruction.
    pub program: String,
    // The data contained from invoking this instruction.
    pub data: Vec<u8>,
    // If this is an inner instruction, we should depend on this
    pub parent_index: i16,
    // The accounts relating to this transaction.
    pub accounts: Vec<AccountInstruction>,
    // The time this log was created in our time
    pub timestamp: i64
}

pub const ACCOUNT_TABLE_NAME: &str = "accounts";

lazy_static! {
    pub static ref ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "account",
        "fields": [
            {"name": "account", "type": "string"},
            {"name": "mint", "type": ["null", "string"]},
            {"name": "owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub struct Account {
    pub account: String,
    pub mint: Option<String>,
    pub owner: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub enum AccountAuthType {
    /// Authority to mint new tokens
    MintTokens = 1,
    /// Authority to freeze any account associated with the Mint
    FreezeAccount = 2,
    /// Owner of a given token account
    AccountOwner = 3,
    /// Authority to close a token account
    CloseAccount = 4,
}

#[derive(Serialize)]
pub struct AccountAuthState {
    /// The account pubkey relating to this auth state.
    pub account: String,
    /// The authority type we're updating
    pub state_type: AccountAuthType,
    /// The new authority
    pub authority: String,
    /// The time this was created in terms of the network
    pub timestamp: i64
}

#[derive(Serialize)]
pub enum TypedDatum {
    BpfLoader(BpfLoaderDatum),
    BpfLoaderUpgradeable(BpfUpgradeableLoaderDatum),
    NativeAssocicatedTokenAccount(NativeAssociatedTokenAccountDatum),
    NativeConfig(NativeConfigDatum),
    NativeLoader(NativeLoaderDatum),
    NativeSecp256k1(NativeSecp256k1Datum),
    NativeStake(NativeStakeDatum),
    NativeSystem(NativeSystemDatum),
    NativeToken(NativeTokenDatum),
    NativeTokenLending(TokenLendingDatum),
    NativeTokenSwap(NativeTokenSwapDatum),
    NativeVote(VoteDatum),
    SerumMarket(SerumMarketDatum),
    SolendTokenLending,
    StepTokenSwap(StepTokenSwapDatum),
    MetaplexAuction(MetaplexAuctionDatum),
    MetaplexAuctionHouse,
    MetaplexCandyMachine,
    Metaplex(MetaplexMainDatum),
    MetaplexTokenMetadata,
    MetaplexTokenVault
}

#[derive(Serialize)]
pub struct TableData {
    pub schema: Schema,
    pub table_name: String,
    pub data: Vec<TypedDatum>
}

/// Derive a simple, singular function that 'decompiles' support program instruction invocations
/// into a database and json-compatible format based on Solana FM's instruction properties.
pub async fn process(
    instructions: Vec<Instruction>,
    transaction: Transaction
) -> Vec<TableData> {
    let instruction_jobs: Vec<_> = instructions
        .into_iter()
        .map(|instruction| {
            let tx = transaction.clone();

            spawn(async move {
                match instruction.program.as_str() {
                    programs::native_associated_token_account::PROGRAM_ADDRESS => {
                        crate::programs::native_associated_token_account::fragment_instruction(
                            instruction).await
                    },
                    programs::native_config::PROGRAM_ADDRESS => {
                        crate::programs::native_config::fragment_instruction(instruction)
                            .await
                    },
                    programs::native_loader::PROGRAM_ADDRESS => {
                        crate::programs::native_loader::fragment_instruction(instruction)
                            .await
                    },
                    programs::bpf_loader::PROGRAM_ADDRESS |
                    programs::bpf_loader::PROGRAM_ADDRESS_2 => {
                        crate::programs::bpf_loader::fragment_instruction(instruction)
                            .await
                    },
                    programs::bpf_loader_upgradeable::PROGRAM_ADDRESS => {
                        crate::programs::bpf_loader_upgradeable::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_secp256k1::PROGRAM_ADDRESS => {
                        crate::programs::native_secp256k1::fragment_instruction(instruction,
                                                                                tx.message.instructions.as_slice())
                            .await
                    }
                    programs::native_stake::PROGRAM_ADDRESS => {
                        crate::programs::native_stake::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_system::PROGRAM_ADDRESS => {
                        crate::programs::native_system::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_token::PROGRAM_ADDRESS => {
                        crate::programs::native_token::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_token_lending::PROGRAM_ADDRESS => {
                        crate::programs::native_token_lending::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_token_swap::PROGRAM_ADDRESS => {
                        crate::programs::native_token_swap::fragment_instruction(instruction)
                            .await
                    }
                    programs::serum_market::PROGRAM_ADDRESS_V1
                        | programs::serum_market::PROGRAM_ADDRESS_V2
                        | programs::serum_market::PROGRAM_ADDRESS_V3 => {
                        crate::programs::serum_market::fragment_instruction(instruction)
                            .await
                    }
                    programs::native_vote::PROGRAM_ADDRESS => {
                        crate::programs::native_vote::fragment_instruction(instruction,
                                                                           tx)
                            .await
                    },
                    programs::step_token_swap::PROGRAM_ADDRESS => {
                        crate::programs::step_token_swap::fragment_instruction(instruction)
                            .await
                    }
                    programs::metaplex_auction::PROGRAM_ADDRESS => {
                        crate::programs::metaplex_auction::fragment_instruction(instruction)
                            .await
                    }
                    programs::metaplex::PROGRAM_ADDRESS => {
                        crate::programs::metaplex::fragment_instruction(instruction)
                            .await
                    }
                    _ => {
                        info!("Looks like this program ({}) is an unsupported one.",
                            instruction.program.to_string());

                        None
                    }
                }
            })
        })
        .collect();

    let mut table_dataset: Vec<TableData> = Vec::new();
    for job in instruction_jobs {
        let res: Result<Option<Vec<TableData>>, _> = job.await;
        if let Ok(data_tables) = res {
            if let Some(data_table_map) = data_tables {
                for table_data in data_table_map {
                    let table_dataset_idx_res = table_dataset.iter().position(|td|
                        td.table_name == table_data.table_name && td.schema == table_data.schema);

                    if let Some(table_dataset_idx) = table_dataset_idx_res {
                        table_dataset[table_dataset_idx].data.extend(table_data.data);
                    } else {
                        table_dataset.push(table_data);
                    }
                }
            }
        }
    }

    table_dataset
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
