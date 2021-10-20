use chrono::NaiveDateTime;
use spl_token_swap::curve::base::CurveType;
use spl_token_swap::instruction::{unpack, SwapInstruction};
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8";

pub async fn process_native_token_swap_instruction(
    // The instruction in question.
    instruction: Instruction
) -> Option<InstructionSet> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = unpack::<SwapInstruction>(
        instruction.data.as_slice());

    if let Ok(token_swap_instruction) = unpack_result {
        return match token_swap_instruction {
            SwapInstruction::Initialize(initialize_instruction) => {
                // The actual calculator will not be indexed.
                // initialize_instruction.swap_curve.calculator

                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "host_fee_numerator".to_string(),
                            value: (&initialize_instruction.fees.host_fee_numerator).to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner_trade_fee_numerator".to_string(),
                            value: (&initialize_instruction.fees.owner_trade_fee_numerator).to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner_trade_fee_denominator".to_string(),
                            value:
                            (&initialize_instruction.fees.owner_trade_fee_denominator)
                                .to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner_withdraw_fee_numerator".to_string(),
                            value:
                            (&initialize_instruction.fees.owner_withdraw_fee_numerator)
                                .to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner_withdraw_fee_denominator".to_string(),
                            value:
                            (&initialize_instruction.fees.owner_withdraw_fee_denominator)
                                .to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "trade_fee_numerator".to_string(),
                            value:
                            (&initialize_instruction.fees.trade_fee_numerator).to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "nonce".to_string(),
                            value: (&initialize_instruction.nonce).to_string(),
                            parent_key: "initialize_instruction".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "trade_fee_denominator".to_string(),
                            value:
                            (&initialize_instruction.fees.trade_fee_denominator).to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "curve_type".to_string(),
                            value: match initialize_instruction.swap_curve.curve_type {
                                CurveType::ConstantProduct => "ConstantProduct".to_string(),
                                /// Flat line, always providing 1:1 from one token to another
                                CurveType::ConstantPrice => "ConstantPrice".to_string(),
                                /// Stable, like uniswap, but with wide zone of 1:1 instead of one point
                                CurveType::Stable => "Stable".to_string(),
                                /// Offset curve, like Uniswap, but the token B side has a faked offset
                                CurveType::Offset => "Offset".to_string(),
                            },
                            parent_key: "swap_curve".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "calculator".to_string(),
                            value: initialize_instruction.swap_curve.calculator.to_string(),
                            parent_key: "swap_curve".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            SwapInstruction::Swap(swap) => {
                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction_index.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "swap".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "amount_in".to_string(),
                            value: swap.amount_in.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "minimum_amount_out".to_string(),
                            value: swap.minimum_amount_out.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
            SwapInstruction::DepositAllTokenTypes(datt) => {
                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "deposit-all-token-types".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "pool_token_amount".to_string(),
                            value: datt.pool_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "maximum_token_a_amount".to_string(),
                            value: datt.maximum_token_a_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "maximum_token_b_amount".to_string(),
                            value: datt.maximum_token_b_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            SwapInstruction::WithdrawAllTokenTypes(watt) => {
                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "withdraw-all-token-types".to_string(),
                        timestamp: instruction.timestamp.clone()
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "pool_token_amount".to_string(),
                            value: watt.pool_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "minimum_token_a_amount".to_string(),
                            value: watt.minimum_token_a_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "minimum_token_b_amount".to_string(),
                            value: watt.minimum_token_b_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            SwapInstruction::DepositSingleTokenTypeExactAmountIn(dstteai) => {
                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "deposit-single-token-type-exact-amount-in".to_string(),
                        timestamp: instruction.timestamp.clone()
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "minimum_pool_token_amount".to_string(),
                            value: dstteai.minimum_pool_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "source_token_amount".to_string(),
                            value: dstteai.source_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            SwapInstruction::WithdrawSingleTokenTypeExactAmountOut(wstteao) => {
                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "withdraw-single-token-type-exact-amount-out".to_string(),
                        timestamp: instruction.timestamp.clone()
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "maximum_pool_token_amount".to_string(),
                            value: wstteao.maximum_pool_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "destination_token_amount".to_string(),
                            value: wstteao.destination_token_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            _ => {
                error!("{}",
                    "[processors/programs/native_token_swap] FATAL: Instruction not supported yet."
                        .to_owned())
            },
        };
    }

    let unpack_error = unpack_result.err().unwrap();
    error!("{}", "[processors/programs/native_token_swap] FATAL: Unrecognised instruction.".to_owned()
            + &stringify_native_token_swap_error(&unpack_error),
    );

    None
}

use spl_token_swap::solana_program::program_error::ProgramError;

pub fn stringify_native_token_swap_error(err: &ProgramError) -> String {
    return match err {
        ProgramError::Custom(custom_err) => {
            "Custom program error: ".to_owned() + &*custom_err.to_string()
        }
        ProgramError::InvalidArgument => "InvalidArgument program error".to_string(),
        ProgramError::InvalidInstructionData => "InvalidInstructionData program error".to_string(),
        ProgramError::InvalidAccountData => "InvalidAccountData program error".to_string(),
        ProgramError::AccountDataTooSmall => "AccountDataTooSmall program error".to_string(),
        ProgramError::InsufficientFunds => "InsufficientFunds program error".to_string(),
        ProgramError::IncorrectProgramId => "IncorrectProgramId program error".to_string(),
        ProgramError::MissingRequiredSignature => {
            "MissingRequiredSignature program error".to_string()
        }
        ProgramError::AccountAlreadyInitialized => {
            "AccountAlreadyInitialized program error".to_string()
        }
        ProgramError::UninitializedAccount => "UninitializedAccount program error".to_string(),
        ProgramError::NotEnoughAccountKeys => "NotEnoughAccountKeys program error".to_string(),
        ProgramError::AccountBorrowFailed => "AccountBorrowFailed program error".to_string(),
        ProgramError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded program error".to_string(),
        ProgramError::InvalidSeeds => "InvalidSeeds program error".to_string(),
        ProgramError::BorshIoError(bio_error) => {
            "BorshIoError program error: ".to_string() + &*bio_error.to_string()
        }
        ProgramError::AccountNotRentExempt => "AccountNotRentExempt program error".to_string(),
        ProgramError::UnsupportedSysvar => "UnsupportedSysvar program error".to_string(),
        _ => "Unknown program error".to_string(),
    };
}