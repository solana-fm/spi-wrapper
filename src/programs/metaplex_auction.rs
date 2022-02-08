use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use serde::Serialize;
use mpl_auction::{ instruction::AuctionInstruction};
use mpl_auction::processor::{PriceFloor, WinnerLimit};
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "auctxRXPeJoc4817jDhf4HbjnhEcr1cCXenosMhK5R8";

pub const METAPLEX_CANCELLED_BIDS_TABLE: &str = "metaplex_auction_cancelled_bids";
pub const METAPLEX_CREATED_AUCTIONS_TABLE: &str = "metaplex_auction_created_auctions";
pub const METAPLEX_CLAIMED_BIDS_TABLE: &str = "metaplex_auction_claimed_bids";
pub const METAPLEX_ENDED_AUCTIONS_TABLE: &str = "metaplex_auction_ended_auctions";
pub const METAPLEX_STARTED_AUCTIONS_TABLE: &str = "metaplex_auction_started_auctions";
pub const METAPLEX_SET_AUTHORITY_TABLE: &str = "metaplex_auction_set_authorities";
pub const METAPLEX_PLACED_BIDS_TABLE: &str = "metaplex_auction_placed_bids";
pub const METAPLEX_CREATED_AUCTIONS_V2_TABLE: &str = "metaplex_auction_created_auctions_v2";

lazy_static! {
    pub static ref METAPLEX_CANCELLED_BID_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_cancelled_bid",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "bidder_token_account", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "pot", "type": "string"},
            {"name": "pot_account", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "auction_account", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CREATED_AUCTIONS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_created_auction",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "extended_data_account", "type": "string"},
            {"name": "winners", "type": "long"},
            {"name": "end_auction_at", "type": ["null", "long"]},
            {"name": "end_auction_gap", "type": ["null", "long"]},
            {"name": "token_mint", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "price_floor", "type": "int"},
            {"name": "tick_size", "type": ["null", "long"]},
            {"name": "gap_tick_size_percentage", "type": ["null", "int"]},
            {"name": "auction_owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CLAIMED_BID_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_claimed_bid",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "bidder_pot_token_account", "type": "string"},
            {"name": "bidder_pot_pda", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "auction_mint", "type": "string"},
            {"name": "token_program", "type": "string"},
            {"name": "auction_extended", "type": "string"},
            {"name": "auction_program", "type": "string"},
            {"name": "store", "type": ["null","string"]},
            {"name": "resource", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_ENDED_AUCTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_ended_auction",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "revealed_price", "type": ["null", "long"]},
            {"name": "revealed_salt", "type": ["null", "long"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_STARTED_AUCTION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_started_auction",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "creator", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_SET_AUTHORITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_set_authority",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "new_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_PLACED_BIDS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_placed_bid",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "bidder", "type": "string"},
            {"name": "bidder_paying_account", "type": "string"},
            {"name": "pot", "type": "string"},
            {"name": "pot_spl", "type": "string"},
            {"name": "metadata", "type": "string"},
            {"name": "token_mint", "type": "string"},
            {"name": "transfer_authority", "type": "string"},
            {"name": "payer", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "resource", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref METAPLEX_CREATED_AUCTIONS_V2_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "metaplex_created_auction_v2",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "auction", "type": "string"},
            {"name": "auction_extended", "type": "string"},
            {"name": "winners", "type": "long"},
            {"name": "end_auction_at", "type": ["null", "long"]},
            {"name": "end_auction_gap", "type": ["null", "long"]},
            {"name": "token_mint", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "resource", "type": "string"},
            {"name": "price_floor", "type": "int"},
            {"name": "tick_size", "type": ["null", "long"]},
            {"name": "gap_tick_size_percentage", "type": ["null", "int"]},
            {"name": "instant_sale_price", "type": ["null", "long"]},
            {"name": "name", "type": ["null", "string"]},
            {"name": "creator", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum MetaplexAuctionDatum {
    CancelBid(CancelledBid),
    CreateAuction(CreatedAuction),
    ClaimBid(ClaimedBid),
    EndAuction(EndedAuction),
    StartAuction(StartedAuction),
    SetAuthority(SetAuthority),
    PlaceBid(PlacedBid),
    CreateAuctionV2(CreatedAuctionV2)
}

/// Struct tables
#[derive(Serialize)]
pub struct CancelledBid {
    pub tx_hash : String,
    /// The bidders primary account
    pub bidder: String,
    /// bidders token account
    pub bidder_token_account: String,
    pub resource: String,
    /// The pot, containing a reference to the stored SPL token account.
    pub pot: String,
    /// The pot SPL account, where the tokens will be deposited.
    pub pot_account: String,
    /// The metadata account, storing information about the bidders actions.
    pub metadata: String,
    /// Auction account, containing data about the auction and item being bid on.
    pub auction_account: String,
    /// Token mint, for transfer instructions and verification.
    pub mint: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreatedAuction {
    pub tx_hash : String,
    /// The account creating the auction, which is authorised to make changes.
    pub auction_owner: String,
    /// Account holding this auction.
    pub auction: String,
    /// Auction extended data account (pda relative to auction of ['auction', program id, vault key, 'extended']).
    pub extended_data_account: String,
    /// How many winners are allowed for this auction. See AuctionData.
    pub winners: i64,
    /// End time is the cut-off point that the auction is forced to end by. See AuctionData.
    pub end_auction_at: Option<i64>,
    /// Gap time is how much time after the previous bid where the auction ends. See AuctionData.
    pub end_auction_gap: Option<i64>,
    /// Token mint for the SPL token used for bidding.
    pub token_mint: String,
    /// Authority
    pub authority: String,
    /// The resource being auctioned. See AuctionData.
    pub resource: String,
    /// Set a price floor.
    pub price_floor: i16,
    /// Add a tick size increment
    pub tick_size: Option<i64>,
    /// Add a minimum percentage increase each bid must meet.
    pub gap_tick_size_percentage: Option<i16>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ClaimedBid {
    pub tx_hash : String,
    pub auction: String,
    /// The destination account
    pub destination: String,
    /// Bidder's wallet
    pub bidder: String,
    /// The bidder pot token account
    pub bidder_pot_token_account: String,
    /// The bidder pot pda account [seed of ['auction', program_id, auction key, bidder key]]
    pub bidder_pot_pda: String,
    pub authority: String,
    /// Token mint of the auction
    pub auction_mint: String,
    /// The token program??
    pub token_program: String,
    /// Auction extended (pda relative to auction of ['auction', program id, vault key, 'extended'])
    pub auction_extended: String,
    pub auction_program: String,
    pub store: Option<String>,
    pub resource: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct EndedAuction {
    pub tx_hash : String,
    /// The auction..
    pub auction: String,
    /// Auction authority
    pub authority: String,
    /// The resource being auctioned. See AuctionData.
    pub resource: String,
    /// If the auction was blinded, a revealing price must be specified to release the auction
    /// winnings.
    pub revealed_price: Option<i64>,
    pub revealed_salt: Option<i64>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct StartedAuction {
    pub tx_hash : String,
    /// The creator/authorised account.
    pub creator: String,
    /// Initialized auction account.
    pub auction: String,
    /// The resource being auctioned. See AuctionData.
    pub resource: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SetAuthority {
    pub tx_hash : String,
    pub auction: String,
    pub authority: String,
    pub new_authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct PlacedBid {
    pub tx_hash : String,
    /// Auction account, containing data about the auction and item being bid on.
    pub auction: String,
    /// The bidders primary account, for PDA calculation/transit auth.
    pub bidder: String,
    /// The bidders token account they'll pay with
    pub bidder_paying_account: String,
    /// The pot, containing a reference to the stored SPL token account.
    pub pot: String,
    /// The pot SPL account, where the tokens will be deposited.
    pub pot_spl: String,
    /// The metadata account, storing information about the bidders actions.
    pub metadata: String,
    /// Token mint, for transfer instructions and verification.
    pub token_mint: String,
    /// Transfer authority, for moving tokens into the bid pot.
    pub transfer_authority: String,
    pub payer: String,
    /// /// Size of the bid being placed. The user must have enough SOL to satisfy this amount.
    pub amount: i64,
    /// Resource being bid on.
    pub resource: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CreatedAuctionV2 {
    pub tx_hash : String,
    pub auction: String,
    pub auction_extended: String,
    /// How many winners are allowed for this auction. See AuctionData.
    pub winners: i64,
    /// End time is the cut-off point that the auction is forced to end by. See AuctionData.
    pub end_auction_at: Option<i64>,
    /// Gap time is how much time after the previous bid where the auction ends. See AuctionData.
    pub end_auction_gap: Option<i64>,
    /// Token mint for the SPL token used for bidding.
    pub token_mint: String,
    /// Authority
    pub authority: String,
    /// The resource being auctioned. See AuctionData.
    pub resource: String,
    /// Set a price floor.
    pub price_floor: i16,
    /// Add a tick size increment
    pub tick_size: Option<i64>,
    /// Add a minimum percentage increase each bid must meet.
    pub gap_tick_size_percentage: Option<i16>,
    /// Add a instant sale price.
    pub instant_sale_price: Option<i64>,
    /// Auction name
    pub name: Option<String>,
    /// The account creating the auction, which is authorised to make changes.
    pub creator: String,
    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    let auction_dr = AuctionInstruction::try_from_slice(&instruction.data);

    return match auction_dr {
        Ok(ref bld) => {
            let deserialized_auction_ix = bld.clone();
            let mut response: Vec<TableData> = Vec::new();
            return match deserialized_auction_ix {
                AuctionInstruction::CancelBid(cancel_bid) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_CANCELLED_BID_SCHEMA).clone(),
                        table_name: METAPLEX_CANCELLED_BIDS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::CancelBid(
                                CancelledBid {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    bidder: instruction.accounts[0].account.to_string(),
                                    bidder_token_account: instruction.accounts[1].account.to_string(),
                                    resource: cancel_bid.resource.to_string(),
                                    pot: instruction.accounts[2].account.to_string(),
                                    pot_account: instruction.accounts[3].account.to_string(),
                                    metadata: instruction.accounts[4].account.to_string(),
                                    auction_account: instruction.accounts[5].account.to_string(),
                                    mint: instruction.accounts[6].account.to_string(),
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::CreateAuction(created_auction) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_CREATED_AUCTIONS_SCHEMA).clone(),
                        table_name: METAPLEX_CREATED_AUCTIONS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::CreateAuction(
                                CreatedAuction {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction_owner: instruction.accounts[0].account.to_string(),
                                    auction: instruction.accounts[1].account.to_string(),
                                    extended_data_account: instruction.accounts[2].account.to_string(),
                                    winners: match created_auction.winners {
                                        WinnerLimit::Unlimited(val) => val,
                                        WinnerLimit::Capped(val) => val
                                    } as i64,
                                    end_auction_at: if let Some(eaa) = created_auction.end_auction_at {
                                        Some(eaa as i64)
                                    } else {
                                        None
                                    },
                                    end_auction_gap: if let Some(eag) = created_auction.end_auction_gap {
                                        Some(eag as i64)
                                    } else {
                                        None
                                    },
                                    token_mint: created_auction.token_mint.to_string(),
                                    authority: created_auction.authority.to_string(),
                                    resource: created_auction.resource.to_string(),
                                    price_floor: match created_auction.price_floor {
                                        PriceFloor::None(_) => 0,
                                        PriceFloor::MinimumPrice(_) => 1,
                                        PriceFloor::BlindedPrice(_) => 2
                                    },
                                    tick_size: if let Some(ts) = created_auction.tick_size {
                                        Some(ts as i64)
                                    } else {
                                        None
                                    },
                                    gap_tick_size_percentage: if let Some(gtsp) = created_auction.gap_tick_size_percentage {
                                        Some(gtsp as i16)
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::ClaimBid(claimed_bid) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_CLAIMED_BID_SCHEMA).clone(),
                        table_name: METAPLEX_CLAIMED_BIDS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::ClaimBid(
                                ClaimedBid {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction: instruction.accounts[4].account.to_string(),
                                    destination: instruction.accounts[0].account.to_string(),
                                    bidder: instruction.accounts[5].account.to_string(),
                                    bidder_pot_token_account: instruction.accounts[1].account.to_string(),
                                    bidder_pot_pda: instruction.accounts[2].account.to_string(),
                                    authority: instruction.accounts[3].account.to_string(),
                                    auction_mint: instruction.accounts[6].account.to_string(),
                                    token_program: instruction.accounts[8].account.to_string(),
                                    auction_extended: instruction.accounts[9].account.to_string(),
                                    auction_program: PROGRAM_ADDRESS.to_string(),
                                    store: None,
                                    resource: claimed_bid.resource.to_string(),
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::EndAuction(ended_auction) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_ENDED_AUCTION_SCHEMA).clone(),
                        table_name: METAPLEX_ENDED_AUCTIONS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::EndAuction(
                                EndedAuction {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction: instruction.accounts[1].account.to_string(),
                                    authority: instruction.accounts[0].account.to_string(),
                                    resource: ended_auction.resource.to_string(),
                                    revealed_price: if let Some(r) = ended_auction.reveal {
                                        Some(r.0 as i64)
                                    } else {
                                        None
                                    },
                                    revealed_salt: if let Some(r) = ended_auction.reveal {
                                        Some(r.1 as i64)
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::StartAuction(started_auction) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_STARTED_AUCTION_SCHEMA).clone(),
                        table_name: METAPLEX_STARTED_AUCTIONS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::StartAuction(
                                StartedAuction {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    creator: instruction.accounts[0].account.to_string(),
                                    auction: instruction.accounts[1].account.to_string(),
                                    resource: started_auction.resource.to_string(),
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::SetAuthority => {
                    let table_data = TableData {
                        schema: (*METAPLEX_SET_AUTHORITY_SCHEMA).clone(),
                        table_name: METAPLEX_SET_AUTHORITY_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::SetAuthority(
                                SetAuthority {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction: instruction.accounts[0].account.to_string(),
                                    authority: instruction.accounts[1].account.to_string(),
                                    new_authority: instruction.accounts[2].account.to_string(),
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::PlaceBid(placed_bid) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_PLACED_BIDS_SCHEMA).clone(),
                        table_name: METAPLEX_PLACED_BIDS_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::PlaceBid(
                                PlacedBid {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction: instruction.accounts[5].account.to_string(),
                                    bidder: instruction.accounts[0].account.to_string(),
                                    bidder_paying_account: instruction.accounts[1].account.to_string(),
                                    pot: instruction.accounts[2].account.to_string(),
                                    pot_spl: instruction.accounts[3].account.to_string(),
                                    metadata: instruction.accounts[4].account.to_string(),
                                    token_mint: instruction.accounts[6].account.to_string(),
                                    transfer_authority: instruction.accounts[7].account.to_string(),
                                    payer: instruction.accounts[8].account.to_string(),
                                    amount: placed_bid.amount as i64,
                                    resource: placed_bid.resource.to_string(),
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                },
                AuctionInstruction::CreateAuctionV2(created_auction) => {
                    let table_data = TableData {
                        schema: (*METAPLEX_CREATED_AUCTIONS_V2_SCHEMA).clone(),
                        table_name: METAPLEX_CREATED_AUCTIONS_V2_TABLE.to_string(),
                        data: vec![TypedDatum::MetaplexAuction(
                            MetaplexAuctionDatum::CreateAuctionV2(
                                CreatedAuctionV2 {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    auction: instruction.accounts[1].account.to_string(),
                                    creator: instruction.accounts[0].account.to_string(),
                                    auction_extended: instruction.accounts[2].account.to_string(),
                                    winners: match created_auction.winners {
                                        WinnerLimit::Unlimited(val) => val,
                                        WinnerLimit::Capped(val) => val
                                    } as i64,
                                    end_auction_at: if let Some(eaa) = created_auction.end_auction_at {
                                        Some(eaa as i64)
                                    } else {
                                        None
                                    },
                                    end_auction_gap: if let Some(eag) = created_auction.end_auction_gap {
                                        Some(eag as i64)
                                    } else {
                                        None
                                    },
                                    token_mint: created_auction.token_mint.to_string(),
                                    authority: created_auction.authority.to_string(),
                                    resource: created_auction.resource.to_string(),
                                    price_floor: match created_auction.price_floor {
                                        PriceFloor::None(_) => 0,
                                        PriceFloor::MinimumPrice(_) => 1,
                                        PriceFloor::BlindedPrice(_) => 2
                                    },
                                    tick_size: if let Some(ts) = created_auction.tick_size {
                                        Some(ts as i64)
                                    } else {
                                        None
                                    },
                                    gap_tick_size_percentage: if let Some(gtsp) = created_auction.gap_tick_size_percentage {
                                        Some(gtsp as i16)
                                    } else {
                                        None
                                    },
                                    instant_sale_price: if let Some(isp) = created_auction.instant_sale_price {
                                        Some(isp as i64)
                                    } else {
                                        None
                                    },
                                    name: if let Some(n) = created_auction.name {
                                        Some(std::str::from_utf8(n.as_slice()).unwrap().to_string())
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
            }
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", instruction.program, err);

            None
        }
    };
}