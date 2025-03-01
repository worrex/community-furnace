use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Updates contract's config, i.e. relevant code_ids, fee_collector address and owner
    UpdateConfig {
        owner: Option<String>,
    },
    Burn {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(LeaderboardResponse)]
    Leaderboard {
        /// An optional parameter specifying what address to start searching after.
        start_after: Option<Addr>,
        /// The amount of incentive contracts to return.
        ///
        /// If unspecified, will default to a value specified by the contract.
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

pub type ConfigResponse = Config;
pub type LeaderboardResponse = Vec<(Addr, Uint128)>;
