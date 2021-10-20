use chrono::NaiveDateTime;
use serum_dex::instruction::MarketInstruction;
use tracing::error;

use crate::{InstructionFunction, InstructionSet, InstructionProperty, Instruction};

pub const PROGRAM_ADDRESS_V1: &str = "BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg";
pub const PROGRAM_ADDRESS_V2: &str = "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o";
pub const PROGRAM_ADDRESS_V3: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";

pub async fn fragment_instruction(
    instruction: Instruction
) -> Option<InstructionSet> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = MarketInstruction::unpack(
        instruction.data.as_slice());

    if let Some(market_instruction) = unpack_result {
        return match market_instruction {
            MarketInstruction::InitializeMarket(imi) => {
                // 0. `[writable]` the market to initialize
                // 1. `[writable]` zeroed out request queue
                // 2. `[writable]` zeroed out event queue
                // 3. `[writable]` zeroed out bids
                // 4. `[writable]` zeroed out asks
                // 5. `[writable]` spl-token account for the coin currency
                // 6. `[writable]` spl-token account for the price currency
                // 7. `[]` coin currency Mint
                // 8. `[]` price currency Mint
                // 9. `[]` the rent sysvar
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "initialize-market".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "coin_lot_size".to_string(),
                            value: imi.coin_lot_size.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "fee_rate_bps".to_string(),
                            value: imi.fee_rate_bps.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "pc_dust_threshold".to_string(),
                            value: imi.pc_dust_threshold.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "pc_lot_size".to_string(),
                            value: imi.pc_lot_size.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "vault_signer_nonce".to_string(),
                            value: imi.vault_signer_nonce.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::NewOrder(noiv1) => {
                // 0. `[writable]` the market
                // 1. `[writable]` the OpenOrders account to use
                // 2. `[writable]` the request queue
                // 3. `[writable]` the (coin or price currency) account paying for the order
                // 4. `[signer]` owner of the OpenOrders account
                // 5. `[writable]` coin vault
                // 6. `[writable]` pc vault
                // 7. `[]` spl token program
                // 8. `[]` the rent sysvar
                // 9. `[writable]` (optional) the (M)SRM account used for fee discounts
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "new-order".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "client_id".to_string(),
                            value: noiv1.client_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit_price".to_string(),
                            value: noiv1.limit_price.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_qty".to_string(),
                            value: noiv1.max_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "order_type".to_string(),
                            value: (noiv1.order_type as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (noiv1.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::MatchOrders(orders) => {
                // 0. `[writable]` market
                // 1. `[writable]` req_q
                // 2. `[writable]` event_q
                // 3. `[writable]` bids
                // 4. `[writable]` asks
                // 5. `[writable]` coin fee receivable account
                // 6. `[writable]` pc fee receivable account
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "match-orders".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "orders".to_string(),
                            value: orders.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
            MarketInstruction::ConsumeEvents(count) => {
                // ... `[writable]` OpenOrders
                // accounts.len() - 4 `[writable]` market
                // accounts.len() - 3 `[writable]` event queue
                // accounts.len() - 2 `[writable]` coin fee receivable account
                // accounts.len() - 1 `[writable]` pc fee receivable account
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "consume-events".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "events".to_string(),
                            value: count.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
            MarketInstruction::CancelOrder(coi) => {
                // 0. `[]` market
                // 1. `[writable]` OpenOrders
                // 2. `[writable]` the request queue
                // 3. `[signer]` the OpenOrders owner

                // #[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
                // #[cfg_attr(test, derive(Arbitrary))]
                // #[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
                // pub struct CancelOrderInstruction {
                //     pub side: Side,
                //     pub order_id: u128,
                //     pub owner: [u64; 4], // Unused
                //     pub owner_slot: u8,
                // }

                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "cancel-order".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (coi.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "order_id".to_string(),
                            value: coi.order_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "owner_slot".to_string(),
                            value: coi.owner_slot.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::SettleFunds => {
                // 0. `[writable]` market
                // 1. `[writable]` OpenOrders
                // 2. `[signer]` the OpenOrders owner
                // 3. `[writable]` coin vault
                // 4. `[writable]` pc vault
                // 5. `[writable]` coin wallet
                // 6. `[writable]` pc wallet
                // 7. `[]` vault signer
                // 8. `[]` spl token program
                // 9. `[writable]` (optional) referrer pc wallet
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "settle-funds".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![],
                })
            }
            MarketInstruction::CancelOrderByClientId(client_id) => {
                // 0. `[]` market
                // 1. `[writable]` OpenOrders
                // 2. `[writable]` the request queue
                // 3. `[signer]` the OpenOrders owner
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "cancel-order-by-client-id".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "client_id".to_string(),
                            value: client_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
            MarketInstruction::DisableMarket => {
                // 0. `[writable]` market
                // 1. `[signer]` disable authority
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "disable-market".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![],
                })
            }
            MarketInstruction::SweepFees => {
                // 0. `[writable]` market
                // 1. `[writable]` pc vault
                // 2. `[signer]` fee sweeping authority
                // 3. `[writable]` fee receivable account
                // 4. `[]` vault signer
                // 5. `[]` spl token program
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "sweep-fees".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![],
                })
            }
            MarketInstruction::NewOrderV2(order) => {
                // 0. `[writable]` the market
                // 1. `[writable]` the OpenOrders account to use
                // 2. `[writable]` the request queue
                // 3. `[writable]` the (coin or price currency) account paying for the order
                // 4. `[signer]` owner of the OpenOrders account
                // 5. `[writable]` coin vault
                // 6. `[writable]` pc vault
                // 7. `[]` spl token program
                // 8. `[]` the rent sysvar
                // 9. `[writable]` (optional) the (M)SRM account used for fee discounts
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "new-order-v2".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "client_id".to_string(),
                            value: order.client_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit_price".to_string(),
                            value: order.limit_price.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_qty".to_string(),
                            value: order.max_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum SelfTradeBehavior {
                        //     DecrementTake = 0,
                        //     CancelProvide = 1,
                        //     AbortTransaction = 2,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "self_trade_behavior".to_string(),
                            value: (order.self_trade_behavior as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum OrderType {
                        //     Limit = 0,
                        //     ImmediateOrCancel = 1,
                        //     PostOnly = 2,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "order_type".to_string(),
                            value: (order.order_type as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum Side {
                        //     Bid = 0,
                        //     Ask = 1,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (order.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::NewOrderV3(order) => {
                // 0. `[writable]` the market
                // 1. `[writable]` the OpenOrders account to use
                // 2. `[writable]` the request queue
                // 3. `[writable]` the event queue
                // 4. `[writable]` bids
                // 5. `[writable]` asks
                // 6. `[writable]` the (coin or price currency) account paying for the order
                // 7. `[signer]` owner of the OpenOrders account
                // 8. `[writable]` coin vault
                // 9. `[writable]` pc vault
                // 10. `[]` spl token program
                // 11. `[]` the rent sysvar
                // 12. `[writable]` (optional) the (M)SRM account used for fee discounts
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "new-order-v3".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "client_order_id".to_string(),
                            value: order.client_order_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit_price".to_string(),
                            value: order.limit_price.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit".to_string(),
                            value: order.limit.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_coin_qty".to_string(),
                            value: order.max_coin_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum SelfTradeBehavior {
                        //     DecrementTake = 0,
                        //     CancelProvide = 1,
                        //     AbortTransaction = 2,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "self_trade_behavior".to_string(),
                            value: (order.self_trade_behavior as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum OrderType {
                        //     Limit = 0,
                        //     ImmediateOrCancel = 1,
                        //     PostOnly = 2,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "order_type".to_string(),
                            value: (order.order_type as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum Side {
                        //     Bid = 0,
                        //     Ask = 1,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (order.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_native_pc_qty_including_fees".to_string(),
                            value: order.max_native_pc_qty_including_fees.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::CancelOrderV2(order) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[signer]` the OpenOrders owner
                // 5. `[writable]` event_q
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "cancel-order-v2".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "order_id".to_string(),
                            value: order.order_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        // pub enum Side {
                        //     Bid = 0,
                        //     Ask = 1,
                        // }
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (order.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::CancelOrderByClientIdV2(client_id) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[signer]` the OpenOrders owner
                // 5. `[writable]` event_q
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "cancel-order-by-client-id-v2".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "client_id".to_string(),
                            value: client_id.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                    ],
                })
            }
            MarketInstruction::SendTake(sti) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[]`
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "send-take".to_string(),
                        timestamp: instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "side".to_string(),
                            value: (sti.side as u8).to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_native_pc_qty_including_fees".to_string(),
                            value: sti.max_native_pc_qty_including_fees.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "max_coin_qty".to_string(),
                            value: sti.max_coin_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit".to_string(),
                            value: sti.limit.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit_price".to_string(),
                            value: sti.limit_price.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "min_coin_qty".to_string(),
                            value: sti.min_coin_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "min_native_pc_qty".to_string(),
                            value: sti.min_native_pc_qty.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
            MarketInstruction::CloseOpenOrders => {
                // 0. `[writable]` OpenOrders
                // 1. `[signer]` the OpenOrders owner
                // 2. `[writable]` the destination account to send rent exemption SOL to
                // 3. `[]` market
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        timestamp: instruction.timestamp,
                        function_name: "close-open-orders".to_string(),
                    },
                    properties: vec![],
                })
            }
            MarketInstruction::InitOpenOrders => {
                // 0. `[writable]` OpenOrders
                // 1. `[signer]` the OpenOrders owner
                // 2. `[writable]` the destination account to send rent exemption SOL to
                // 3. `[]` market
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        timestamp: instruction.timestamp,
                        function_name: "init-open-orders".to_string(),
                    },
                    properties: vec![],
                })
            }
            MarketInstruction::Prune(limit) => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        timestamp: instruction.timestamp,
                        function_name: "prune".to_string(),
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "limit".to_string(),
                            value: limit.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        }
                    ],
                })
            }
        };
    }

    error!("{}", "[processors/programs/serum/market] FATAL: Unrecognised instruction.".to_string());
    None
}
