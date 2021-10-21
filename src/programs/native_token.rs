use spl_token::instruction::TokenInstruction;
use spl_token::solana_program::program_option::COption;
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    // We don't have anything to work with
    let tdr = TokenInstruction::unpack(instruction.data.as_slice());

    return if let Ok(dti) = tdr {
        match dti {
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => {
                // Source code
                // msg!("Instruction: InitializeMint");
                // Self::process_initialize_mint(
                //     accounts,
                //     decimals,
                //     mint_authority,
                //     freeze_authority,
                // )
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-mint".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "decimals".to_string(),
                            value: decimals.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "mint_authority".to_string(),
                            value: mint_authority.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "freeze_authority".to_string(),
                            value: if let COption::Some(fa) = freeze_authority {
                                fa.to_string()
                            } else {
                                "".to_string()
                            },
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::InitializeAccount => {
                // msg!("Instruction: InitializeAccount");
                // Self::process_initialize_account(accounts)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-account".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            TokenInstruction::InitializeAccount2 { owner } => {
                // msg!("Instruction: InitializeAccount2");
                // Self::process_initialize_account2(accounts, owner)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-account-2".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::InitializeMultisig { m } => {
                // msg!("Instruction: InitializeMultisig");
                // Self::process_initialize_multisig(accounts, m)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-multisig".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "m".to_string(),
                            value: m.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::Transfer { amount } => {
                // msg!("Instruction: Transfer");
                // Self::process_transfer(program_id, accounts, amount, None)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "transfer".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::Approve { amount } => {
                // msg!("Instruction: Approve");
                // Self::process_approve(program_id, accounts, amount, None)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "approve".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::Revoke => {
                // msg!("Instruction: Revoke");
                // Self::process_revoke(program_id, accounts)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "revoke".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => {
                // msg!("Instruction: SetAuthority");
                // Self::process_set_authority(
                //     program_id,
                //     accounts,
                //     authority_type,
                //     new_authority,
                // )
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "set-authority".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "authority_type".to_string(),
                            value: (authority_type as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "new_authority".to_string(),
                            value: if let COption::Some(na) = new_authority {
                                na.to_string()
                            } else {
                                "".to_string()
                            },
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::MintTo { amount } => {
                // msg!("Instruction: MintTo");
                // Self::process_mint_to(program_id, accounts, amount, None)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "mint-to".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::Burn { amount } => {
                // msg!("Instruction: Burn");
                // Self::process_burn(program_id, accounts, amount, None)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "burn".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::CloseAccount => {
                // msg!("Instruction: CloseAccount");
                // Self::process_close_account(program_id, accounts)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "close-account".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            TokenInstruction::FreezeAccount => {
                // msg!("Instruction: FreezeAccount");
                // Self::process_toggle_freeze_account(program_id, accounts, true)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "freeze-account".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            TokenInstruction::ThawAccount => {
                // msg!("Instruction: ThawAccount");
                // Self::process_toggle_freeze_account(program_id, accounts, false)
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "thaw-account".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            TokenInstruction::TransferChecked { amount, decimals } => {
                // msg!("Instruction: TransferChecked");
                // Self::process_transfer(program_id, accounts, amount, Some(decimals))
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "transfer-checked".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "decimals".to_string(),
                            value: decimals.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::ApproveChecked { amount, decimals } => {
                // msg!("Instruction: ApproveChecked");
                // Self::process_approve(program_id, accounts, amount, Some(decimals))
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "approve-checked".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "decimals".to_string(),
                            value: decimals.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::MintToChecked { amount, decimals } => {
                // msg!("Instruction: MintToChecked");
                // Self::process_mint_to(program_id, accounts, amount, Some(decimals))
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "mint-to-checked".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "decimals".to_string(),
                            value: decimals.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::BurnChecked { amount, decimals } => {
                // msg!("Instruction: BurnChecked");
                // Self::process_burn(program_id, accounts, amount, Some(decimals))
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "burn-checked".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "decimals".to_string(),
                            value: decimals.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ]
                })
            }
            TokenInstruction::SyncNative => {
                // msg!("Instruction: SyncNative");
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "sync-native".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
        }
    } else {
        error!("{}", "Invalid instruction for the token program.".to_string());
        None
    };
}