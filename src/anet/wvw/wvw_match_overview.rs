use ::prelude::*;

gw2rs_id_string!(WVWMatchOverviewId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverview {
    id: WVWMatchOverviewId,
    worlds: Vec<WVWMatchOverviewWorlds>,
    all_worlds: Vec<LinkedWVWServers>,
    start_time: String, // TODO: Timestamp
    end_time: String // TODO: Timestamp
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverviewWorlds {
    red: WorldId,
    blue: WorldId,
    green: WorldId
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchOverviewAllWorlds {
    red: Vec<WorldId>,
    green: Vec<WorldId>,
    blue: Vec<WorldId>
}
