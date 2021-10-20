mod programs;

use chrono::NaiveDateTime;
use tokio::spawn;
use tracing::info;

#[derive(Clone)]
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
    // The time this log was created in our time
    pub timestamp: NaiveDateTime,
}

#[derive(Clone)]
pub struct InstructionFunction {
    // The local unique identifier of the instruction according to the transaction (not based on solana)
    pub tx_instruction_id: i16,
    // The transaction this instruction belongs to.
    pub transaction_hash: String,
    // If this is an inner instruction, we should depend on this
    pub parent_index: i16,
    // Which program does this function belong to?
    pub program: String,
    // Which function is this function? (Well duh)
    pub function_name: String,
    // Like what it means dude.
    pub timestamp: NaiveDateTime
}

#[derive(Clone)]
pub struct InstructionProperty {
    // The local unique identifier of the instruction according to the transaction (not based on solana)
    pub tx_instruction_id: i16,
    // The local unique identifier of the instruction type (not based on solana)
    pub transaction_hash: String,
    // If this is an inner instruction, we should depend on this
    pub parent_index: i16,
    pub key: String,
    pub value: String,
    pub parent_key: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone)]
pub struct InstructionSet {
    pub function: InstructionFunction,
    pub properties: Vec<InstructionProperty>
}

/// Derive a simple, singular function that 'decompiles' support program instruction invocations
/// into a database and json-compatible format based on Solana FM's instruction properties.
pub async fn process(
    instructions: Vec<Instruction>
) -> Vec<InstructionSet> {
    let instruction_jobs: Vec<_> = instructions.into_iter()
        .map(|instruction| {
            spawn(async {
                match instruction.program.as_str() {
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
                    _ => {
                        info!("Looks like this program ({}) is an unsupported one.",
                            instruction.program.to_string());

                        None
                    }
                }
            })
        })
        .collect();

    let mut instruction_sets: Vec<InstructionSet> = Vec::new();
    for job in instruction_jobs {
        let res = job.await;
        if let Ok(instruction_job_result) = res {
            if let Some(instruction_set) = instruction_job_result {
                instruction_sets.push(instruction_set);
            }
        }
    }

    instruction_sets
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}