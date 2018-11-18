use ::prelude::*;

gw2rs_id_string!(WVWMatchId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchScores {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchServers {
    pub red: WorldId,
    pub green: WorldId,
    pub blue: WorldId,
}

#[derive(Debug, Deserialize)]
pub struct LinkedWVWServers {
    pub red: Vec<WorldId>,
    pub green: Vec<WorldId>,
    pub blue: Vec<WorldId>,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchDeaths {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchKills {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchVictoryPoints {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmishScores {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmishMapScores {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchMapScores {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMapKills {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Debug, Deserialize)]
pub struct WVWMapDeaths {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

/// The `WVWMatch` struct represents all associated data related to a World versus World matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatch {
    pub id: WVWMatchId,
    pub start_time: String,
    pub end_time: String,
    pub scores: WVWMatchScores,
    pub worlds: WVWMatchServers,
    pub all_worlds: LinkedWVWServers,
    pub deaths: WVWMatchDeaths,
    pub kills: WVWMatchKills,
    pub victory_points: WVWMatchVictoryPoints,
    pub skirmishes: Vec<WVWMatchSkirmish>,
    pub maps: Vec<WVWMatchMap>,
}

/// The `WVWMatchSkirmish` struct represents skirmish data for particular matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmish {
    pub id: u64,
    pub scores: WVWMatchSkirmishScores,
    pub map_scores: Vec<WVWMatchSkirmishMapScoreData>,
}

/// The `WVWMatchSkirmishMapScoreData` struct contains the individual map scores for a given
/// Skirmish.
#[derive(Debug, Deserialize)]
pub struct WVWMatchSkirmishMapScoreData {
    #[serde(rename = "type")]
    pub map: WVWMap,
    pub scores: WVWMatchSkirmishMapScores,
}

/// The `WVWSkirmishMap` struct details map specific data for a particular matchup.
#[derive(Debug, Deserialize)]
pub struct WVWMatchMap {
    pub id: u64,
    #[serde(rename = "type")]
    pub map: WVWMap,
    pub scores: WVWMatchMapScores,
    pub bonuses: Vec<WVWBonus>,
    pub objectives: Vec<WVWObjective>,
    pub kills: WVWMapKills,
    pub deaths: WVWMapDeaths,
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
    pub bonus_type: WVWBonusType,
    pub owner: WVWBonusOwner,
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
    pub id: WVWObjectiveId,
    #[serde(rename = "type")]
    pub objective_type: WVWObjectiveType,
    pub last_flipped: String, // TODO: Timestamp
    pub claimed_by: GuildId,
    pub claimed_at: String, // TODO: timestamp
    pub points_tick: u64,
    pub points_capture: u64,
    pub yaks_delivered: u64,
    pub guild_upgrades: Vec<GuildUpgradeId>,
}