use bincode::deserialize;
use solana_program::{
    instruction::CompiledInstruction, pubkey::Pubkey, system_instruction::SystemInstruction,
};
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "11111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction
) -> Option<InstructionSet> {
    let sdr = deserialize::<SystemInstruction>(&data);

    return if let Ok(si) = sdr {
        match si {
            SystemInstruction::CreateAccount {
                lamports,
                space,
                owner,
            } => {
                // check_num_system_accounts(&instruction.accounts, 2)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "createAccount".to_string(),
                //     info: json!({
                //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "lamports": lamports,
                //         "space": space,
                //         "owner": owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "create-account".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "lamports".to_string(),
                            value: lamports.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "space".to_string(),
                            value: space.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                    ],
                })
            }
            SystemInstruction::Assign { owner } => {
                // check_num_system_accounts(&instruction.accounts, 1)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "assign".to_string(),
                //     info: json!({
                //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "owner": owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "assign".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::Transfer { lamports } => {
                // check_num_system_accounts(&instruction.accounts, 2)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "transfer".to_string(),
                //     info: json!({
                //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "lamports": lamports,
                //     }),
                // })
                // check_num_system_accounts(&instruction.accounts, 2)?;
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "transfer".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "lamports".to_string(),
                            value: lamports.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::CreateAccountWithSeed {
                base,
                seed,
                lamports,
                space,
                owner,
            } => {
                // check_num_system_accounts(&instruction.accounts, 2)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "createAccountWithSeed".to_string(),
                //     info: json!({
                //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "base": base.to_string(),
                //         "seed": seed,
                //         "lamports": lamports,
                //         "space": space,
                //         "owner": owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "create-account-with-seed".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "base".to_string(),
                            value: base.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "seed".to_string(),
                            value: seed.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "lamports".to_string(),
                            value: lamports.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "space".to_string(),
                            value: space.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::AdvanceNonceAccount => {
                // check_num_system_accounts(&instruction.accounts, 3)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "advanceNonce".to_string(),
                //     info: json!({
                //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "nonceAuthority": account_keys[instruction.accounts[2] as usize].to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "advance-nonce-account".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![],
                })
            }
            SystemInstruction::WithdrawNonceAccount(lamports) => {
                // check_num_system_accounts(&instruction.accounts, 5)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "withdrawFromNonce".to_string(),
                //     info: json!({
                //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "recentBlockhashesSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                //         "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                //         "nonceAuthority": account_keys[instruction.accounts[4] as usize].to_string(),
                //         "lamports": lamports,
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "withdraw-nonce-account".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "lamports".to_string(),
                            value: lamports.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::InitializeNonceAccount(authority) => {
                // check_num_system_accounts(&instruction.accounts, 3)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "initializeNonce".to_string(),
                //     info: json!({
                //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "rentSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                //         "nonceAuthority": authority.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-nonce-account".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "authority".to_string(),
                            value: authority.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::AuthorizeNonceAccount(authority) => {
                // check_num_system_accounts(&instruction.accounts, 1)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "authorizeNonce".to_string(),
                //     info: json!({
                //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "nonceAuthority": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "newAuthorized": authority.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "authorize-nonce-account".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "authority".to_string(),
                            value: authority.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::Allocate { space } => {
                // check_num_system_accounts(&instruction.accounts, 1)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "allocate".to_string(),
                //     info: json!({
                //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "space": space,
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "allocate".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "space".to_string(),
                            value: space.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::AllocateWithSeed {
                base,
                seed,
                space,
                owner,
            } => {
                // check_num_system_accounts(&instruction.accounts, 2)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "allocateWithSeed".to_string(),
                //     info: json!({
                //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "base": base.to_string(),
                //         "seed": seed,
                //         "space": space,
                //         "owner": owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "allocate-with-seed".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "base".to_string(),
                            value: base.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "seed".to_string(),
                            value: seed.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "space".to_string(),
                            value: space.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::AssignWithSeed { base, seed, owner } => {
                // check_num_system_accounts(&instruction.accounts, 2)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "assignWithSeed".to_string(),
                //     info: json!({
                //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "base": base.to_string(),
                //         "seed": seed,
                //         "owner": owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "assign-with-seed".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "base".to_string(),
                            value: base.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "seed".to_string(),
                            value: seed.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
            SystemInstruction::TransferWithSeed {
                lamports,
                from_seed,
                from_owner,
            } => {
                // check_num_system_accounts(&instruction.accounts, 3)?;
                // Ok(ParsedInstructionEnum {
                //     instruction_type: "transferWithSeed".to_string(),
                //     info: json!({
                //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                //         "sourceBase": account_keys[instruction.accounts[1] as usize].to_string(),
                //         "destination": account_keys[instruction.accounts[2] as usize].to_string(),
                //         "lamports": lamports,
                //         "sourceSeed": from_seed,
                //         "sourceOwner": from_owner.to_string(),
                //     }),
                // })
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "transfer-with-seed".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "lamports".to_string(),
                            value: lamports.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "from_seed".to_string(),
                            value: from_seed.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "from_owner".to_string(),
                            value: from_owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ],
                })
            }
        }
    } else {
        error!("{}", "[spi-wrapper/programs/bpf_loader] Error deserializing this system \
        instruction!".to_string());

        None
    };
}