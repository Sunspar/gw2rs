use ::prelude::*;

gw2rs_id_string!(WVWMatchScoreId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchScore {
    id: WVWMatchScoreId,
    scores: Vec<WVWMatchMapScore>,
    maps: Vec<WVWMatchScoreMap>
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchMapScore {
    red: u64,
    green: u64,
    blue: u64
}

gw2rs_id_string!(WVWMatchScoreMapId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchScoreMap {
    id: WVWMatchScoreMapId,
    #[serde(rename = "type")]
    map_type: WVWMap,
    scores: WVWMatchScoreMapScore
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchScoreMapScore {
    red: u64,
    green: u64,
    blue: u64
}