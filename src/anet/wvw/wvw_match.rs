macro_rules! gw2rs_wvw_server_data {
    ($name: ident, $ty: ty) => {
        #[derive(Debug, Deserialize)]
        pub struct $name {
            red: $ty,
            green: $ty,
            blue: $ty,
        }
    };
}

use crate::prelude::*;

gw2rs_id_string!(WVWMatchId);
gw2rs_wvw_server_data!(WVWMatchScores, u64);
gw2rs_wvw_server_data!(WVWMatchServers, WorldId);
gw2rs_wvw_server_data!(
    LinkedWVWServers,
    Vec<
        WorldId,
    >
);
gw2rs_wvw_server_data!(WVWMatchDeaths, u64);
gw2rs_wvw_server_data!(WVWMatchKills, u64);
gw2rs_wvw_server_data!(WVWMatchVictoryPoints, u64);
gw2rs_wvw_server_data!(WVWMatchSkirmishScores, u64);
gw2rs_wvw_server_data!(WVWMatchSkirmishMapScores, u64);
gw2rs_wvw_server_data!(WVWMatchMapScores, u64);
gw2rs_wvw_server_data!(WVWMapKills, u64);
gw2rs_wvw_server_data!(WVWMapDeaths, u64);

use crate::anet::wvw::map::WVWMap;

/// The `WVWMatch` struct represents all associated data related to a World versus World matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatch {
    id: WVWMatchId,
    start_time: String,
    end_time: String,
    scores: WVWMatchScores,
    worlds: WVWMatchServers,
    all_worlds: LinkedWVWServers,
    deaths: WVWMatchDeaths,
    kills: WVWMatchKills,
    victory_points: WVWMatchVictoryPoints,
    skirmishes: Vec<WVWMatchSkirmish>,
    maps: Vec<WVWMatchMap>,
}

/// The `WVWMatchSkirmish` struct represents skirmish data for particular matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmish {
    id: u64,
    scores: WVWMatchSkirmishScores,
    map_scores: Vec<WVWMatchSkirmishMapScoreData>,
}

/// The `WVWMatchSkirmishMapScoreData` struct contains the individual map scores for a given
/// Skirmish.
#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmishMapScoreData {
    #[serde(rename = "type")]
    map: WVWMap,
    scores: WVWMatchSkirmishMapScores,
}

/// The `WVWSkirmishMap` struct details map specific data for a particular matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatchMap {
    id: u64,
    #[serde(rename = "type")]
    map: WVWMap,
    scores: WVWMatchMapScores,
    bonuses: Vec<WVWBonus>,
    objectives: Vec<WVWObjective>,
    kills: WVWMapKills,
    deaths: WVWMapDeaths,
}

#[derive(Debug, Deserialize)]
pub enum WVWBonusType {
    Bloodlust,
}
#[derive(Debug, Deserialize)]
pub enum WVWBonusOwner {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Deserialize)]
pub struct WVWBonus {
    bonus_type: WVWBonusType,
    owner: WVWBonusOwner,
}

#[derive(Debug, Deserialize)]
pub enum WVWObjectiveType {
    Camp,
    Castle,
    Keep,
    Mercenary,
    Tower,
    Ruins,
    Resource,
    Generic,
    Spawn,
}

gw2rs_id_u64!(GuildUpgradeId);
gw2rs_id_u64!(WVWObjectiveId);
gw2rs_id_u64!(GuildId);

#[derive(Debug, Deserialize)]
pub struct WVWObjective {
    id: WVWObjectiveId,
    #[serde(rename = "type")]
    objective_type: WVWObjectiveType,
    last_flipped: String, // TODO: Timestamp
    claimed_by: GuildId,
    claimed_at: String, // TODO: timestamp
    points_tick: u64,
    points_capture: u64,
    yaks_delivered: u64,
    guild_upgrades: Vec<GuildUpgradeId>,
}
