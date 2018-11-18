use ::prelude::*;

gw2rs_id_string!(WVWMatchOverviewId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverview {
    pub id: WVWMatchOverviewId,
    pub worlds: WVWMatchOverviewWorlds,
    pub all_worlds: LinkedWVWServers,
    pub start_time: String, // TODO: Timestamp
    pub end_time: String // TODO: Timestamp
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverviewWorlds {
    pub red: WorldId,
    pub blue: WorldId,
    pub green: WorldId
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverviewAllWorlds {
    pub red: Vec<WorldId>,
    pub green: Vec<WorldId>,
    pub blue: Vec<WorldId>
}
