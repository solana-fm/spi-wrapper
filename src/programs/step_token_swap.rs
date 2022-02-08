use avro_rs::Schema;
use serde::Serialize;
use solana_program::program_error::ProgramError;
use step_token_swap::instruction::{unpack, SwapInstruction, DeregisterPool};
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};
use crate::programs::native_token::*;
use crate::StepTokenSwapDatum::{Flow, NewRegistry, NewSwapPeg, PegFeeUpdate, RepairCFA, Swap};

pub const PROGRAM_ADDRESS: &str = "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1";

pub const STEP_TOKEN_SWAP_PEG_TABLE: &str = "step_token_swap_pegs";
pub const STEP_TOKEN_SWAP_FEE_UPDATE_TABLE: &str = "step_token_swap_peg_fee_updates";
pub const STEP_TOKEN_SWAP_SWAP_TABLE: &str = "step_token_swap_swaps";
pub const STEP_TOKEN_SWAP_PEG_FLOW_TABLE: &str = "step_token_swap_peg_flows";
pub const STEP_TOKEN_SWAP_NEW_REGISTRY_TABLE: &str = "step_token_swap_new_registries";
pub const STEP_TOKEN_SWAP_POOL_DEREGISTER_TABLE: &str = "step_token_swap_pool_deregisters";
pub const STEP_TOKEN_SWAP_REPAIR_CFA_TABLE: &str = "step_token_swap_cfa_repairs";

lazy_static! {
    pub static ref STEP_TOKEN_SWAP_PEG_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_peg",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "peg_pda", "type": "string"},
            {"name": "curve_type", "type": "int"},
            {"name": "nonce", "type": "int"},
            {"name": "pool_nonce", "type": "int"},
            {"name": "swap_authority", "type": "string"},
            {"name": "token_a_account", "type": "string"},
            {"name": "token_b_account", "type": "string"},
            {"name": "peg_supply_account", "type": "string"},
            {"name": "pool_mint", "type": "string"},
            {"name": "fee_receiving_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_FEE_UPDATE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_peg_fee_update",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "swap_location", "type": "string"},
            {"name": "trade_fee_numerator", "type": "long"},
            {"name": "trade_fee_denominator", "type": "long"},
            {"name": "owner_trade_fee_numerator", "type": "long"},
            {"name": "owner_trade_fee_denominator", "type": "long"},
            {"name": "owner_withdraw_fee_numerator", "type": "long"},
            {"name": "owner_withdraw_fee_denominator", "type": "long"},
            {"name": "host_fee_numerator", "type": "long"},
            {"name": "host_fee_denominator", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_SWAP_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_swap",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "swap_type", "type": "int"},
            {"name": "source", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "user", "type": "string"},
            {"name": "source_token", "type": "string"},
            {"name": "destination_token", "type": "string"},
            {"name": "amount_in", "type": "long"},
            {"name": "minimum_amount_out", "type": "long"},
            {"name": "flags", "type": "int"},
            {"name": "pool_mint", "type": "string"},
            {"name": "pool_fee_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_PEG_FLOW_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_peg_flow",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "flow_type", "type": "int"},
            {"name": "peg_pda", "type": "string"},
            {"name": "swap_authority", "type": "string"},
            {"name": "user", "type": "string"},
            {"name": "source", "type": "string"},
            {"name": "target", "type": "string"},
            {"name": "pool_token_amount", "type": "long"},
            {"name": "token_amount", "type": "long"},
            {"name": "pool_account", "type": "string"},
            {"name": "pool_mint_account", "type": "string"},
            {"name": "pool_fee_account", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_NEW_REGISTRY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_new_registry",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "deployer", "type": "string"},
            {"name": "registry", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_POOL_DEREGISTER_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_pool_deregister",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "pool_idx", "type": "long"},
            {"name": "deployer", "type": "string"},
            {"name": "registry", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref STEP_TOKEN_SWAP_REPAIR_CFA_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "step_token_swap_repair_cfa
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "token_swap_account", "type": "string"},
            {"name": "old_fee_account_to_close", "type": "string"},
            {"name": "new_fee_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum StepTokenSwapDatum {
    NewSwapPeg(Peg),
    PegFeeUpdate(PegFee),
    Swap(StepSwap),
    Flow(PegFlow),
    NewRegistry(Registry),
    DeregisterPool(PoolDeregister),
    RepairCFA(RepairClosedFeeAccount)
}

#[derive(Clone, Serialize)]
pub enum CurveType {
    /// Uniswap-style constant product curve, invariant = token_a_amount * token_b_amount
    ConstantProduct = 0,
    /// Flat line, always providing 1:1 from one token to another
    ConstantPrice = 1,
    /// Stable, like uniswap, but with wide zone of 1:1 instead of one point
    Stable = 2,
    /// Offset curve, like Uniswap, but the token B side has a faked offset
    Offset = 3,
}

impl CurveType {
    pub fn from_native(curve_type: step_token_swap::curve::base::CurveType) -> Self {
        match curve_type {
            step_token_swap::curve::base::CurveType::ConstantProduct => CurveType::ConstantProduct,
            step_token_swap::curve::base::CurveType::ConstantPrice => CurveType::ConstantPrice,
            step_token_swap::curve::base::CurveType::Stable => CurveType::Stable,
            step_token_swap::curve::base::CurveType::Offset => CurveType::Offset
        }
    }
}

/// A peg = a new swap pegging between token A and B.
#[derive(Serialize)]
pub struct Peg {
    pub tx_hash : String,
    /// The peg's program derived account, account that stores the peg data (i.e. SRM-SOL)
    pub peg_pda: String,
    pub curve_type: i16,
    /// nonce used to create valid program address
    pub nonce: i16,
    /// nonce used to create valid program address for the pool
    pub pool_nonce: i16,
    /// The authority of the swap, basically the program that handles this swap.
    pub swap_authority: String,
    pub token_a_account: String,
    pub token_b_account: String,
    /// Account to deposit the initial pool token supply. Usually index [7]
    pub peg_supply_account: String,
    pub pool_mint: String,
    /// Account to deposit trading and withdraw fees. Usually index [6]
    pub fee_receiving_account: String,
    pub timestamp: i64,
}

/// Tracks the changes/updates made to the Peg's fees.
#[derive(Serialize)]
pub struct PegFee {
    pub tx_hash : String,
    /// The PDA of the token-swap in question.
    pub swap_location: String,
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
    pub timestamp: i64,
}

#[derive(Serialize)]
pub enum SwapType {
    Normal = 0,
    Routed = 1,
}

#[derive(Serialize)]
pub struct StepSwap {
    pub tx_hash : String,
    pub swap_type: i16,
    /// The authority of the swap, basically the user that started this swap.
    pub source: String,
    /// DESTINATION Account assigned to USER as the owner.
    pub destination: String,
    /// User transfer authority
    pub user: String,
    /// What token is the user sending?
    /// Is usually -> token_(A|B) Base Account to swap FROM.  Must be the DESTINATION token.
    pub source_token: String,
    /// What token is the user receiving?
    /// Is usually -> token_(A|B) DESTINATION Account assigned to USER as the owner.
    pub destination_token: String,
    /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: i64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: i64,
    /// Flags defining swap behavior; see SwapFlags
    pub flags: i16,
    /// Pool token mint, to generate trading fees
    pub pool_mint: String,
    /// Pool fee account
    pub pool_fee_account: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub enum FlowType {
    Deposit = 0,
    Withdraw = 1,
}

#[derive(Serialize)]
pub struct PegFlow {
    pub tx_hash : String,
    /// Which side? Deposit or Withdraw..
    pub flow_type: i16,
    pub peg_pda: String,
    pub swap_authority: String,
    /// user transfer authority
    pub user: String,
    /// Usually "user transfer authority can transfer amount"
    pub source: String,
    /// Usually "Base Account to deposit into."
    pub target: String,
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub pool_token_amount: i64,
    /// token amount to (Maximum if deposit, Minimum if withdraw) , prevents excessive slippage
    pub token_amount: i64,
    /// DEPOSIT: Pool Account to deposit the generated tokens, user is the owner.
    /// WITHDRAWAL: SOURCE Pool account, amount is transferable by user transfer authority.
    pub pool_account: String,
    /// Pool token mint, to generate trading fees
    pub pool_mint_account: String,
    /// Pool fee account
    pub pool_fee_account: Option<String>,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct Registry {
    pub tx_hash : String,
    pub deployer: String,
    pub registry: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct PoolDeregister {
    pub tx_hash : String,
    pub pool_idx: i64,
    pub deployer: String,
    pub registry: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct RepairClosedFeeAccount {
    pub tx_hash : String,
    pub token_swap_account: String,
    pub old_fee_account_to_close: String,
    pub new_fee_account: String,
    pub timestamp: i64
}

pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = step_token_swap::instruction::SwapInstruction::unpack(
        instruction.data.as_slice());

    return match unpack_result {
        Ok(ref tsi) => {
            let mut response: Vec<TableData> = Vec::new();
            let token_swap_instruction = tsi.clone();
            match token_swap_instruction {
                /// A new swap (like literally THE new SOL/SRM for example, would happen here once.
                SwapInstruction::Initialize(initialize) => {
                    // The actual calculator will not be indexed.
                    // initialize_instruction.swap_curve.calculator
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_PEG_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_PEG_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            NewSwapPeg(Peg {
                                tx_hash: instruction.transaction_hash.to_string(),
                                peg_pda: instruction.accounts[1].account.to_string(),
                                curve_type: CurveType::from_native(initialize.swap_curve.curve_type) as i16,
                                nonce: initialize.nonce as i16,
                                pool_nonce: initialize.pool_nonce as i16,
                                swap_authority: instruction.accounts[2].account.to_string(),
                                token_a_account: instruction.accounts[3].account.to_string(),
                                token_b_account: instruction.accounts[4].account.to_string(),
                                peg_supply_account: instruction.accounts[7].account.to_string(),
                                pool_mint: instruction.accounts[5].account.to_string(),
                                fee_receiving_account: instruction.accounts[6].account.to_string(),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_FEE_UPDATE_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_FEE_UPDATE_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            PegFeeUpdate(PegFee {
                                tx_hash: instruction.transaction_hash.to_string(),
                                swap_location: instruction.accounts[1].account.to_string(),
                                trade_fee_numerator: initialize.fees.trade_fee_numerator as i64,
                                trade_fee_denominator: initialize.fees.trade_fee_denominator as i64,
                                owner_trade_fee_numerator: initialize.fees.owner_trade_fee_numerator as i64,
                                owner_trade_fee_denominator: initialize.fees.owner_trade_fee_denominator as i64,
                                owner_withdraw_fee_numerator: initialize.fees.owner_withdraw_fee_numerator as i64,
                                owner_withdraw_fee_denominator: initialize.fees.owner_withdraw_fee_denominator as i64,
                                host_fee_numerator: -1,
                                host_fee_denominator: -1,
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                SwapInstruction::Swap(swap) => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_SWAP_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_SWAP_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            Swap(StepSwap {
                                tx_hash: instruction.transaction_hash.to_string(),
                                swap_type: SwapType::Normal as i16,
                                source: instruction.accounts[3].account.to_string(),
                                destination: instruction.accounts[6].account.to_string(),
                                user: instruction.accounts[2].account.to_string(),
                                source_token: instruction.accounts[4].account.to_string(),
                                destination_token: instruction.accounts[5].account.to_string(),
                                amount_in: swap.amount_in as i64,
                                minimum_amount_out: swap.minimum_amount_out as i64,
                                flags: swap.flags as i16,
                                pool_mint: instruction.accounts[7].account.to_string(),
                                pool_fee_account: instruction.accounts[8].account.to_string(),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                SwapInstruction::DepositAllTokenTypes(datt) => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_PEG_FLOW_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_PEG_FLOW_TABLE.to_string(),
                        data: vec![
                            TypedDatum::StepTokenSwap(
                                Flow(PegFlow {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    flow_type: 0,
                                    peg_pda: instruction.accounts[0].account.to_string(),
                                    swap_authority: instruction.accounts[1].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source: instruction.accounts[3].account.to_string(),
                                    target: instruction.accounts[5].account.to_string(),
                                    pool_token_amount: (datt.pool_token_amount / 2) as i64,
                                    token_amount: datt.maximum_token_a_amount as i64,
                                    pool_account: instruction.accounts[8].account.to_string(),
                                    pool_mint_account: instruction.accounts[7].account.to_string(),
                                    pool_fee_account: None,
                                    timestamp: instruction.timestamp,
                                })
                            ),
                            TypedDatum::StepTokenSwap(
                                Flow(PegFlow {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    flow_type: 0,
                                    peg_pda: instruction.accounts[0].account.to_string(),
                                    swap_authority: instruction.accounts[1].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source: instruction.accounts[4].account.to_string(),
                                    target: instruction.accounts[6].account.to_string(),
                                    pool_token_amount: (datt.pool_token_amount - datt.pool_token_amount / 2) as i64,
                                    token_amount: datt.maximum_token_b_amount as i64,
                                    pool_account: instruction.accounts[8].account.to_string(),
                                    pool_mint_account: instruction.accounts[7].account.to_string(),
                                    pool_fee_account: None,
                                    timestamp: instruction.timestamp,
                                })
                            )],
                    });

                    Some(response)
                }
                SwapInstruction::WithdrawAllTokenTypes(watt) => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_PEG_FLOW_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_PEG_FLOW_TABLE.to_string(),
                        data: vec![
                            TypedDatum::StepTokenSwap(
                                Flow(PegFlow {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    flow_type: 1,
                                    peg_pda: instruction.accounts[0].account.to_string(),
                                    swap_authority: instruction.accounts[1].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source: instruction.accounts[5].account.to_string(),
                                    target: instruction.accounts[7].account.to_string(),
                                    // https://stackoverflow.com/a/9842149/6811682
                                    pool_token_amount: (watt.pool_token_amount / 2) as i64,
                                    token_amount: watt.minimum_token_a_amount as i64,
                                    pool_account: instruction.accounts[4].account.to_string(),
                                    pool_mint_account: instruction.accounts[3].account.to_string(),
                                    pool_fee_account: Some(instruction.accounts[9].account.to_string()),
                                    timestamp: instruction.timestamp,
                                })
                            ),
                            TypedDatum::StepTokenSwap(
                                Flow(PegFlow {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    flow_type: 1,
                                    peg_pda: instruction.accounts[0].account.to_string(),
                                    swap_authority: instruction.accounts[1].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source: instruction.accounts[6].account.to_string(),
                                    target: instruction.accounts[8].account.to_string(),
                                    // https://stackoverflow.com/a/9842149/6811682
                                    pool_token_amount: (watt.pool_token_amount - watt.pool_token_amount / 2) as i64,
                                    token_amount: watt.minimum_token_b_amount as i64,
                                    pool_account: instruction.accounts[4].account.to_string(),
                                    pool_mint_account: instruction.accounts[3].account.to_string(),
                                    pool_fee_account: Some(instruction.accounts[9].account.to_string()),
                                    timestamp: instruction.timestamp,
                                })
                            )],
                    });

                    Some(response)
                }
                // TODO: Derive which token is deposited via inner instructions?
                SwapInstruction::DepositSingleTokenTypeExactAmountIn(dstteai) => {
                    // response.push(TableData {
                    //     schema: (*STEP_TOKEN_SWAP_PEG_FLOW_SCHEMA).clone(),
                    //     table_name: STEP_TOKEN_SWAP_PEG_FLOW_TABLE.to_string(),
                    //     data: vec![
                    //         TypedDatum::StepTokenSwap(
                    //             Flow(PegFlow {
                    //                 flow_type: 0,
                    //                 peg_pda: instruction.accounts[0].account.to_string(),
                    //                 swap_authority: instruction.accounts[1].account.to_string(),
                    //                 user: instruction.accounts[2].account.to_string(),
                    //                 source: instruction.accounts[3].account.to_string(),
                    //                 target: instruction.accounts[4].account.to_string(),
                    //                 pool_account: instruction.accounts[7].account.to_string(),
                    //                 pool_token_amount: dstteai.source_token_amount as i64,
                    //                 token_amount: dstteai.minimum_pool_token_amount as i64,
                    //                 pool_mint_account: instruction.accounts[6].account.to_string(),
                    //                 pool_fee_account: None,
                    //                 timestamp: instruction.timestamp,
                    //             })
                    //         )
                    //     ],
                    // });
                    //
                    // Some(response)
                    None
                }
                // TODO: Derive which token is withdrawn via inner instructions?
                SwapInstruction::WithdrawSingleTokenTypeExactAmountOut(wstteao) => {
                    // response.push(TableData {
                    //     schema: (*STEP_TOKEN_SWAP_PEG_SCHEMA).clone(),
                    //     table_name: STEP_TOKEN_SWAP_PEG_TABLE.to_string(),
                    //     data: vec![
                    //         TypedDatum::StepTokenSwap(
                    //             Flow(PegFlow {
                    //                 flow_type: 1,
                    //                 peg_pda: instruction.accounts[0].account.to_string(),
                    //                 swap_authority: instruction.accounts[1].account.to_string(),
                    //                 user: instruction.accounts[2].account.to_string(),
                    //                 source: instruction.accounts[5].account.to_string(),
                    //                 target: instruction.accounts[7].account.to_string(),
                    //                 /// Maximum amount of pool tokens to burn. User receives an output of token A
                    //                 /// or B based on the percentage of the pool tokens that are returned.
                    //                 pool_token_amount: wstteao.maximum_pool_token_amount as i64,
                    //                 /// Amount of token A or B to receive
                    //                 token_amount: wstteao.destination_token_amount as i64,
                    //                 pool_account: instruction.accounts[4].account.to_string(),
                    //                 pool_mint_account: instruction.accounts[3].account.to_string(),
                    //                 pool_fee_account: Some(instruction.accounts[8].account.to_string()),
                    //                 timestamp: instruction.timestamp,
                    //             })
                    //         ),
                    //         TypedDatum::StepTokenSwap(
                    //             Flow(PegFlow {
                    //                 flow_type: 1,
                    //                 peg_pda: instruction.accounts[0].account.to_string(),
                    //                 swap_authority: instruction.accounts[1].account.to_string(),
                    //                 user: instruction.accounts[2].account.to_string(),
                    //                 source: instruction.accounts[6].account.to_string(),
                    //                 target: instruction.accounts[7].account.to_string(),
                    //                 /// Maximum amount of pool tokens to burn. User receives an output of token A
                    //                 /// or B based on the percentage of the pool tokens that are returned.
                    //                 pool_token_amount: wstteao.maximum_pool_token_amount as i64,
                    //                 /// Amount of token A or B to receive
                    //                 token_amount: wstteao.destination_token_amount as i64,
                    //                 pool_account: instruction.accounts[4].account.to_string(),
                    //                 pool_mint_account: instruction.accounts[3].account.to_string(),
                    //                 pool_fee_account: Some(instruction.accounts[8].account.to_string()),
                    //                 timestamp: instruction.timestamp,
                    //             })
                    //         )
                    //     ]
                    // });
                    //
                    // Some(response)
                    None
                }
                SwapInstruction::InitializeRegistry() => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_NEW_REGISTRY_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_NEW_REGISTRY_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            NewRegistry(Registry {
                                tx_hash: instruction.transaction_hash.to_string(),
                                deployer: instruction.accounts[0].account.to_string(),
                                registry: instruction.accounts[1].account.to_string(),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                SwapInstruction::RoutedSwap(swap) => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_SWAP_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_SWAP_TABLE.to_string(),
                        data: vec![
                            TypedDatum::StepTokenSwap(
                                Swap(StepSwap {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    swap_type: SwapType::Routed as i16,
                                    source: instruction.accounts[3].account.to_string(),
                                    destination: instruction.accounts[6].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source_token: instruction.accounts[4].account.to_string(),
                                    destination_token: instruction.accounts[5].account.to_string(),
                                    amount_in: swap.amount_in as i64,
                                    minimum_amount_out: 0,
                                    flags: swap.flags as i16,
                                    pool_mint: instruction.accounts[7].account.to_string(),
                                    pool_fee_account: instruction.accounts[8].account.to_string(),
                                    timestamp: instruction.timestamp,
                                })
                            ),
                            TypedDatum::StepTokenSwap(
                                Swap(StepSwap {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    swap_type: SwapType::Routed as i16,
                                    source: instruction.accounts[6].account.to_string(),
                                    destination: instruction.accounts[14].account.to_string(),
                                    user: instruction.accounts[2].account.to_string(),
                                    source_token: instruction.accounts[12].account.to_string(),
                                    destination_token: instruction.accounts[13].account.to_string(),
                                    amount_in: 0,
                                    minimum_amount_out: swap.minimum_amount_out as i64,
                                    flags: swap.flags as i16,
                                    pool_mint: instruction.accounts[15].account.to_string(),
                                    pool_fee_account: instruction.accounts[16].account.to_string(),
                                    timestamp: instruction.timestamp,
                                })
                            )
                        ],
                    });

                    Some(response)
                }
                SwapInstruction::DeregisterPool(deregister_pool) => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_POOL_DEREGISTER_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_POOL_DEREGISTER_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            StepTokenSwapDatum::DeregisterPool(PoolDeregister {
                                tx_hash: instruction.transaction_hash.to_string(),
                                pool_idx: deregister_pool.pool_index as i64,
                                registry: instruction.accounts[1].account.to_string(),
                                deployer: instruction.accounts[0].account.to_string(),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                SwapInstruction::RepairClosedFeeAccount() => {
                    response.push(TableData {
                        schema: (*STEP_TOKEN_SWAP_REPAIR_CFA_SCHEMA).clone(),
                        table_name: STEP_TOKEN_SWAP_REPAIR_CFA_TABLE.to_string(),
                        data: vec![TypedDatum::StepTokenSwap(
                            RepairCFA(RepairClosedFeeAccount {
                                tx_hash: instruction.transaction_hash.to_string(),
                                token_swap_account: instruction.accounts[0].account.to_string(),
                                old_fee_account_to_close: instruction.accounts[1].account.to_string(),
                                new_fee_account: instruction.accounts[2].account.to_string(),
                                timestamp: instruction.timestamp
                            })
                        )],
                    });

                    Some(response)
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
                ProgramError::IllegalOwner => "IllegalOwner".to_string(),
                ProgramError::AccountsDataBudgetExceeded => "AccountsDataBudgetExceeded".to_string(),
            };

            error!("{} Reason: {}", "[processors/programs/native_token_swap] FATAL: Unrecognised instruction.".to_string(),
            err_msg);

            None
        }
    };
}