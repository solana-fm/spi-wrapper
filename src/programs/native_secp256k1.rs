use bincode::deserialize;
use sha3::Digest;
use libsecp256k1::PublicKey;
use solana_sdk::secp256k1_instruction::{
    construct_eth_pubkey, Secp256k1Error, SecpSignatureOffsets, HASHED_PUBKEY_SERIALIZED_SIZE,
    SIGNATURE_OFFSETS_SERIALIZED_SIZE, SIGNATURE_SERIALIZED_SIZE,
};
use tracing::{error, info};

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "KeccakSecp256k11111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
    // The instructions that were part of the transaction, in order.
    instructions: &[solana_program::instruction::Instruction]
) -> Option<InstructionSet> {
    // The first element within data slice tells us the number of signatures.
    let count = instruction.data[0] as usize;
    let expected_data_size = 1 + count * SIGNATURE_OFFSETS_SERIALIZED_SIZE;

    // Create the instruction
    let mut interpreted_instruction_set = InstructionSet {
        function: InstructionFunction {
            tx_instruction_id: instruction.tx_instruction_id.clone(),
            transaction_hash: instruction.transaction_hash.clone(),
            parent_index: instruction.parent_index.clone(),
            program: instruction.program.clone(),
            function_name: "".to_string(),
            timestamp: instruction.timestamp,
        },
        properties: vec![],
    };

    // On-chain failed instruction detected, safe anyways.
    if instruction.data.len() < expected_data_size {
        info!(
            "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found -> Secp256k1Error::InvalidInstructionDataSize"
        );

        return Some(interpreted_instruction_set)
    }

    // Gather every instruction's data.
    // Adapted from: https://github.com/solana-labs/solana/blob/cab30e2356b1badd78e2285d10e95adaceb7ceed/sdk/src/transaction.rs#L398
    let instruction_datas: Vec<&[u8]> = instructions
        .iter()
        .map(|instruction| instruction.data.as_ref())
        .collect();

    for i in 0..count {
        let start = 1 + i * SIGNATURE_OFFSETS_SERIALIZED_SIZE;
        let end = start + SIGNATURE_OFFSETS_SERIALIZED_SIZE;

        let offsets_result = bincode::deserialize::<SecpSignatureOffsets>(
            &instruction.data[start..end]);

        if let Ok(offsets) = offsets_result {
            // Parse out signature
            let signature_index = offsets.signature_instruction_index as usize;
            if signature_index >= instruction_datas.len() {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the signature_index was greater than the data size. -> \
        Secp256k1Error::InvalidInstructionDataSize"
                );
                return Some(interpreted_instruction_set)
            }
            let signature_instruction = instruction_datas[signature_index];
            let sig_start = offsets.signature_offset as usize;
            let sig_end = sig_start + SIGNATURE_SERIALIZED_SIZE;
            if sig_end >= signature_instruction.len() {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the last_signature_index was greater than the data size. -> \
        Secp256k1Error::InvalidSignature"
                );
                return Some(interpreted_instruction_set)
            }
            let signature_result = libsecp256k1::Signature::parse_standard_slice(
                &signature_instruction[sig_start..sig_end],
            )
                .map_err(|_| Secp256k1Error::InvalidSignature);
            if let Err(_) = signature_result {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the signature's slice cannot be parsed. -> Secp256k1Error::InvalidSignature"
                );
                return Some(interpreted_instruction_set)
            }
            let signature = signature_result.unwrap();

            let recovery_id_result =
                libsecp256k1::RecoveryId::parse(signature_instruction[sig_end])
                    .map_err(|_| Secp256k1Error::InvalidRecoveryId);
            if let Err(_) = recovery_id_result {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the signature's slice cannot be parsed. -> Secp256k1Error::InvalidSignature"
                );
                return Some(interpreted_instruction_set)
            }
            let recovery_id = recovery_id_result.unwrap();

            // Parse out pubkey
            let eth_address_slice_result = get_data_slice(
                &instruction_datas,
                offsets.eth_address_instruction_index,
                offsets.eth_address_offset,
                HASHED_PUBKEY_SERIALIZED_SIZE,
            );
            if let Err(_) = eth_address_slice_result {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the eth address slice cannot be parsed."
                );
                return Some(interpreted_instruction_set)
            }
            let eth_address_slice = eth_address_slice_result.unwrap();

            // Parse out message
            let message_slice_result = get_data_slice(
                &instruction_datas,
                offsets.message_instruction_index,
                offsets.message_data_offset,
                offsets.message_data_size as usize,
            );
            if let Err(_) = message_slice_result {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the message slice cannot be parsed."
                );
                return Some(interpreted_instruction_set)
            }
            let message_slice = message_slice_result.unwrap();

            let mut hasher = sha3::Keccak256::new();
            hasher.update(message_slice);
            let message_hash = hasher.finalize();

            let pubkey_result: Result<PublicKey, Secp256k1Error> = libsecp256k1::recover(
                &libsecp256k1::Message::parse_slice(&message_hash).unwrap(),
                &signature,
                &recovery_id,
            )
                .map_err(|_| Secp256k1Error::InvalidSignature);
            if let Err(_) = pubkey_result {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because the pubkey's slice cannot be parsed. -> Secp256k1Error::InvalidSignature"
                );
                return Some(interpreted_instruction_set)
            }
            let pubkey: libsecp256k1::PublicKey = pubkey_result.unwrap();

            let eth_address = construct_eth_pubkey(&pubkey);
            let eth_address_str = "0x".to_string() + &*hex::encode(eth_address);
            interpreted_instruction_set.properties
                .push(InstructionProperty {
                    tx_instruction_id: instruction.tx_instruction_id.clone(),
                    transaction_hash: instruction.transaction_hash.clone(),
                    parent_index: instruction.parent_index.clone(),
                    key: "eth_address".to_string(),
                    value: eth_address_str.to_string(),
                    parent_key: "".to_string(),
                    timestamp: instruction.timestamp.clone(),
                });

            if eth_address_slice != eth_address {
                info!(
                    "[spi-wrapper/programs/native_secp256k1] INFO: On-chain failed instruction \
        found because there was an eth address slice mismatch v.s. the actual eth address. -> \
        Secp256k1Error::InvalidSignature"
                );
                return Some(interpreted_instruction_set)
            }

            return Some(interpreted_instruction_set)
        }
    }

    error!("{}",
        "[spi-wrapper/programs/native_secp256k1] FATAL: The instruction interpretation \
    terminated unknowingly."
            .to_string(),
    );

    None
}

// Adapted from secp256k1_instruction
// https://github.com/solana-labs/solana/blob/d269ca510cc9961be9bdc7ae09574e44cfd713a3/sdk/src/secp256k1_instruction.rs#L176
fn get_data_slice<'a>(
    instruction_datas: &'a [&[u8]],
    instruction_index: u8,
    offset_start: u16,
    size: usize,
) -> Result<&'a [u8], Secp256k1Error> {
    let signature_index = instruction_index as usize;
    if signature_index >= instruction_datas.len() {
        return Err(Secp256k1Error::InvalidDataOffsets);
    }
    let signature_instruction = &instruction_datas[signature_index];
    let start = offset_start as usize;
    let end = start + size;
    if end > signature_instruction.len() {
        return Err(Secp256k1Error::InvalidSignature);
    }

    Ok(&instruction_datas[signature_index][start..end])
}