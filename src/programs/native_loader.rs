use solana_program::instruction::CompiledInstruction;

use crate::InstructionProperty;

const PROGRAM_ADDRESS: &str = "NativeLoader1111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: CompiledInstruction,
) -> Vec<InstructionProperty> {
    // We don't have anything to work with
    Vec::new()
}