use spl_token_swap::curve::base::CurveType;
use spl_token_swap::instruction::{unpack, SwapInstruction};
use spl_token_swap::solana_program::program_error::ProgramError;
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8";

pub async fn fragment_instruction(
    // The instruction in question.
    instruction: Instruction
) -> Option<InstructionSet> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = unpack::<SwapInstruction>(
        instruction.data.as_slice());

    return match unpack_result {
        Ok(ref tsi) => {
            let token_swap_instruction = tsi.clone();
            match token_swap_instruction {
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
                                parent_index: instruction.parent_index.clone(),
                                key: "host_fee_numerator".to_string(),
                                value: (&initialize_instruction.fees.host_fee_numerator).to_string(),
                                parent_key: "fees".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "owner_trade_fee_numerator".to_string(),
                                value: (&initialize_instruction.fees.owner_trade_fee_numerator).to_string(),
                                parent_key: "fees".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "trade_fee_numerator".to_string(),
                                value:
                                (&initialize_instruction.fees.trade_fee_numerator).to_string(),
                                parent_key: "fees".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "nonce".to_string(),
                                value: (&initialize_instruction.nonce).to_string(),
                                parent_key: "initialize_instruction".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "trade_fee_denominator".to_string(),
                                value:
                                (&initialize_instruction.fees.trade_fee_denominator).to_string(),
                                parent_key: "fees".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "curve_type".to_string(),
                                value: match initialize_instruction.swap_curve.curve_type {
                                    CurveType::ConstantProduct => "ConstantProduct".to_string(),
                                    // Flat line, always providing 1:1 from one token to another
                                    CurveType::ConstantPrice => "ConstantPrice".to_string(),
                                    // Stable, like uniswap, but with wide zone of 1:1 instead of one point
                                    CurveType::Stable => "Stable".to_string(),
                                    // Offset curve, like Uniswap, but the token B side has a faked offset
                                    CurveType::Offset => "Offset".to_string(),
                                },
                                parent_key: "swap_curve".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            // InstructionProperty {
                            //     tx_instruction_id: instruction.tx_instruction_id.clone(),
                            //     transaction_hash: instruction.transaction_hash.clone(),
                            //     parent_index: instruction.parent_index.clone(),
                            //     key: "calculator".to_string(),
                            //     value: initialize_instruction.swap_curve.calculator.to_string(),
                            //     parent_key: "swap_curve".to_string(),
                            //     timestamp: instruction.timestamp.clone(),
                            // },
                        ],
                    })
                }
                SwapInstruction::Swap(swap) => {
                    Option::from(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "amount_in".to_string(),
                                value: swap.amount_in.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "pool_token_amount".to_string(),
                                value: datt.pool_token_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "maximum_token_a_amount".to_string(),
                                value: datt.maximum_token_a_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "pool_token_amount".to_string(),
                                value: watt.pool_token_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "minimum_token_a_amount".to_string(),
                                value: watt.minimum_token_a_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "minimum_pool_token_amount".to_string(),
                                value: dstteai.minimum_pool_token_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
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
                                parent_index: instruction.parent_index.clone(),
                                key: "maximum_pool_token_amount".to_string(),
                                value: wstteao.maximum_pool_token_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "destination_token_amount".to_string(),
                                value: wstteao.destination_token_amount.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
            }
        }
        Err(err) => {
            let err_msg = match err {
                ProgramError::Custom(_) => "Custom".to_string(),
                ProgramError::InvalidArgument => "InvalidArgument".to_string(),
                ProgramError::InvalidInstructionData => "InvalidInstructionData".to_string(),
                ProgramError::InvalidAccountData => "InvalidAccountData".to_string(),
                ProgramError::AccountDataTooSmall => "AccountDataTooSmall".to_string(),
                ProgramError::InsufficientFunds => "InsufficientFunds".to_string(),
                ProgramError::IncorrectProgramId => "IncorrectProgramId".to_string(),
                ProgramError::MissingRequiredSignature => "MissingRequiredSignature".to_string(),
                ProgramError::AccountAlreadyInitialized => "AccountAlreadyInitialized".to_string(),
                ProgramError::UninitializedAccount => "UninitializedAccount".to_string(),
                ProgramError::NotEnoughAccountKeys => "NotEnoughAccountKeys".to_string(),
                ProgramError::AccountBorrowFailed => "AccountBorrowFailed".to_string(),
                ProgramError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded".to_string(),
                ProgramError::InvalidSeeds => "InvalidSeeds".to_string(),
                ProgramError::BorshIoError(_) => "BorshIoError".to_string(),
                ProgramError::AccountNotRentExempt => "AccountNotRentExempt".to_string(),
                ProgramError::UnsupportedSysvar => "UnsupportedSysvar".to_string(),
                ProgramError::IllegalOwner => "IllegalOwner".to_string()
            };

            error!("{} Reason: {}", "[processors/programs/native_token_swap] FATAL: Unrecognised instruction.".to_string(),
            err_msg);

            None
        }
    }
}