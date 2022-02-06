use avro_rs::Schema;
use serde::Serialize;
use spl_token_swap::instruction::{unpack, SwapInstruction};

use crate::{Instruction, TableData};

pub const PROGRAM_ADDRESS: &str = "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8";

pub const NATIVE_TOKEN_SWAP_PEG_TABLE: &str = "native_token_swap_pegs";

lazy_static! {
    pub static ref NATIVE_TOKEN_SWAP_PEG_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_token_swap_peg",
        "fields": [
            {"name": "market_account", "type": "string"},
            {"name": "token_program", "type": "string"},
            {"name": "oracle_program", "type": "string"},
            {"name": "quote_currency", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum NativeTokenSwapDatum {
    None
}

/// A peg = a new swap pegging between token A and B.
#[derive(Serialize)]
pub struct Peg {
    /// The pegging account, account that stores the peg data (i.e. SRM-SOL)
    pub token_swap: String,
    /// The authority of the swap, basically the program that handles this swap.
    pub swap_authority: String,
    pub token_a_mint: String,
    pub token_b_mint: String,
    pub pool_account: String,
    pub fee_account: String,
    pub timestamp: i64
}

pub struct PegFee {
    /// Trade fees are extra token amounts that are held inside the token
    /// accounts during a trade, making the value of liquidity tokens rise.
    /// Trade fee numerator
    pub trade_fee_numerator: i64,
    /// Trade fee denominator
    pub trade_fee_denominator: i64,

    /// Owner trading fees are extra token amounts that are held inside the token
    /// accounts during a trade, with the equivalent in pool tokens minted to
    /// the owner of the program.
    /// Owner trade fee numerator
    pub owner_trade_fee_numerator: i64,
    /// Owner trade fee denominator
    pub owner_trade_fee_denominator: i64,

    /// Owner withdraw fees are extra liquidity pool token amounts that are
    /// sent to the owner on every withdrawal.
    /// Owner withdraw fee numerator
    pub owner_withdraw_fee_numerator: i64,
    /// Owner withdraw fee denominator
    pub owner_withdraw_fee_denominator: i64,

    /// Host fees are a proportion of the owner trading fees, sent to an
    /// extra account provided during the trade.
    /// Host trading fee numerator
    pub host_fee_numerator: i64,
    /// Host trading fee denominator
    pub host_fee_denominator: i64,
}

#[derive(Serialize)]
pub struct Swap {
    /// The authority of the swap, basically the user that started this swap.
    /// Is usually -> "token_(A|B) Base Account to swap INTO.  Must be the SOURCE token.", amount is transferable by $authority,
    pub source: String,
    /// What token is the user sending?
    /// Is usually -> token_(A|B) Base Account to swap FROM.  Must be the DESTINATION token.
    pub token_a_provider_account: String,
    /// What token is the user receiving?
    /// Is usually -> token_(A|B) DESTINATION Account assigned to USER as the owner.
    pub token_b_provider_account: String,
    pub amount_in: i64,
    pub minimum_amount_out: i64,
    pub timestamp: i64,
}

#[deprecated(since="0.0.4", note="new revamp to come soon, does not seem viable due to backwards \
incompatibility.")]
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = unpack::<SwapInstruction>(
        instruction.data.as_slice());

    // return match unpack_result {
    //     Ok(ref tsi) => {
    //         let token_swap_instruction = tsi.clone();
    //         match token_swap_instruction {
    //             /// A new swap (like literally THE new SOL/SRM for example, would happen here once.
    //             SwapInstruction::Initialize(initialize) => {
    //                 // The actual calculator will not be indexed.
    //                 // initialize_instruction.swap_curve.calculator
    //
    //                 None
    //             }
    //             SwapInstruction::Swap(swap) => {
    //                 let swap = Swap {
    //                     source: instruction.account_instructions.into_iter()
    //                         .filter(|ai| ai.index == 4)
    //                         .collect(),
    //                     token_a_provider_account: "".to_string(),
    //                     token_b_provider_account: "".to_string(),
    //                     amount_in: swap.amount_in as i64,
    //                     minimum_amount_out: swap.minimum_amount_out as i64,
    //                     timestamp: instruction.timestamp
    //                 };
    //
    //                 let key =
    //                     (NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_INFLATION_SCHEMA);
    //                 let mint_to = MintInflation {
    //                     account: instruction.account_instructions.into_iter()
    //                         .filter(|ai| ai.index == 1)
    //                         .collect(),
    //                     mint: instruction.account_instructions.into_iter()
    //                         .filter(|ai| ai.index == 0)
    //                         .collect(),
    //                     amount: amount as i64,
    //                     decimals: None,
    //                     owner: instruction.account_instructions.into_iter()
    //                         .filter(|ai| ai.index == 2)
    //                         .collect(),
    //                     timestamp: instruction.timestamp
    //                 };
    //
    //                 if response.keys().any(|&k| &k == &key) {
    //                     response[&key].push(mint_to);
    //                 } else {
    //                     response[&key] = vec![mint_to];
    //                 }
    //
    //                 Some(response)
    //             }
    //             SwapInstruction::DepositAllTokenTypes(datt) => {
    //                 // Option::from(InstructionSet {
    //                 //     function: InstructionFunction {
    //                 //         tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //         transaction_hash: instruction.transaction_hash.clone(),
    //                 //         parent_index: instruction.parent_index.clone(),
    //                 //         program: instruction.program.clone(),
    //                 //         function_name: "deposit-all-token-types".to_string(),
    //                 //         timestamp: instruction.timestamp.clone(),
    //                 //     },
    //                 //     properties: vec![
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "pool_token_amount".to_string(),
    //                 //             value: datt.pool_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "maximum_token_a_amount".to_string(),
    //                 //             value: datt.maximum_token_a_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "maximum_token_b_amount".to_string(),
    //                 //             value: datt.maximum_token_b_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //     ],
    //                 // })
    //                 None
    //             }
    //             SwapInstruction::WithdrawAllTokenTypes(watt) => {
    //                 // Option::from(InstructionSet {
    //                 //     function: InstructionFunction {
    //                 //         tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //         transaction_hash: instruction.transaction_hash.clone(),
    //                 //         parent_index: instruction.parent_index.clone(),
    //                 //         program: instruction.program.clone(),
    //                 //         function_name: "withdraw-all-token-types".to_string(),
    //                 //         timestamp: instruction.timestamp.clone()
    //                 //     },
    //                 //     properties: vec![
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "pool_token_amount".to_string(),
    //                 //             value: watt.pool_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "minimum_token_a_amount".to_string(),
    //                 //             value: watt.minimum_token_a_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "minimum_token_b_amount".to_string(),
    //                 //             value: watt.minimum_token_b_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //     ],
    //                 // })
    //                 None
    //             }
    //             SwapInstruction::DepositSingleTokenTypeExactAmountIn(dstteai) => {
    //                 // Option::from(InstructionSet {
    //                 //     function: InstructionFunction {
    //                 //         tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //         transaction_hash: instruction.transaction_hash.clone(),
    //                 //         parent_index: instruction.parent_index.clone(),
    //                 //         program: instruction.program.clone(),
    //                 //         function_name: "deposit-single-token-type-exact-amount-in".to_string(),
    //                 //         timestamp: instruction.timestamp.clone()
    //                 //     },
    //                 //     properties: vec![
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "minimum_pool_token_amount".to_string(),
    //                 //             value: dstteai.minimum_pool_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "source_token_amount".to_string(),
    //                 //             value: dstteai.source_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //     ],
    //                 // })
    //                 None
    //             }
    //             SwapInstruction::WithdrawSingleTokenTypeExactAmountOut(wstteao) => {
    //                 // Option::from(InstructionSet {
    //                 //     function: InstructionFunction {
    //                 //         tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //         transaction_hash: instruction.transaction_hash.clone(),
    //                 //         parent_index: instruction.parent_index.clone(),
    //                 //         program: instruction.program.clone(),
    //                 //         function_name: "withdraw-single-token-type-exact-amount-out".to_string(),
    //                 //         timestamp: instruction.timestamp.clone()
    //                 //     },
    //                 //     properties: vec![
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "maximum_pool_token_amount".to_string(),
    //                 //             value: wstteao.maximum_pool_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //         InstructionProperty {
    //                 //             tx_instruction_id: instruction.tx_instruction_id.clone(),
    //                 //             transaction_hash: instruction.transaction_hash.clone(),
    //                 //             parent_index: instruction.parent_index.clone(),
    //                 //             key: "destination_token_amount".to_string(),
    //                 //             value: wstteao.destination_token_amount.to_string(),
    //                 //             parent_key: "".to_string(),
    //                 //             timestamp: instruction.timestamp.clone(),
    //                 //         },
    //                 //     ],
    //                 // })
    //                 None
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         let err_msg = match err {
    //             ProgramError::Custom(_) => "Custom".to_string(),
    //             ProgramError::InvalidArgument => "InvalidArgument".to_string(),
    //             ProgramError::InvalidInstructionData => "InvalidInstructionData".to_string(),
    //             ProgramError::InvalidAccountData => "InvalidAccountData".to_string(),
    //             ProgramError::AccountDataTooSmall => "AccountDataTooSmall".to_string(),
    //             ProgramError::InsufficientFunds => "InsufficientFunds".to_string(),
    //             ProgramError::IncorrectProgramId => "IncorrectProgramId".to_string(),
    //             ProgramError::MissingRequiredSignature => "MissingRequiredSignature".to_string(),
    //             ProgramError::AccountAlreadyInitialized => "AccountAlreadyInitialized".to_string(),
    //             ProgramError::UninitializedAccount => "UninitializedAccount".to_string(),
    //             ProgramError::NotEnoughAccountKeys => "NotEnoughAccountKeys".to_string(),
    //             ProgramError::AccountBorrowFailed => "AccountBorrowFailed".to_string(),
    //             ProgramError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded".to_string(),
    //             ProgramError::InvalidSeeds => "InvalidSeeds".to_string(),
    //             ProgramError::BorshIoError(_) => "BorshIoError".to_string(),
    //             ProgramError::AccountNotRentExempt => "AccountNotRentExempt".to_string(),
    //             ProgramError::UnsupportedSysvar => "UnsupportedSysvar".to_string(),
    //             ProgramError::IllegalOwner => "IllegalOwner".to_string()
    //         };
    //
    //         error!("{} Reason: {}", "[processors/programs/native_token_swap] FATAL: Unrecognised instruction.".to_string(),
    //         err_msg);
    //
    //         None
    //     }
    // }

    None
}