use ::prelude::*;

gw2rs_id_string!(WVWMatchScoreId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchScore {
    pub id: WVWMatchScoreId,
    pub scores: Vec<WVWMatchMapScore>,
    pub maps: Vec<WVWMatchScoreMap>
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchMapScore {
    pub red: u64,
    pub green: u64,
    pub blue: u64
}

gw2rs_id_string!(WVWMatchScoreMapId);

#[derive(Debug, Deserialize)]
pub struct WVWMatchScoreMap {
    pub id: WVWMatchScoreMapId,
    #[serde(rename = "type")]
    pub map_type: WVWMap,
    pub scores: WVWMatchScoreMapScore
}

#[derive(Debug, Deserialize)]
pub struct WVWMatchScoreMapScore {
    pub red: u64,
    pub green: u64,
    pub blue: u64
}