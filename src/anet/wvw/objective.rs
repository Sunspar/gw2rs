use ::prelude::*;

gw2rs_id_string!(ObjectiveId);

/// `ObjectiveType`s provide instances of the set of possible objective types.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ObjectiveType {
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

/// `Objective`s represent capturable or interactable objective points in World vs World maps.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Objective {
    pub id: ObjectiveId,
    pub name: String,
    #[serde(rename = "type")]
    pub objective_type: ObjectiveType,
    pub sector_id: u64, // TODO: Newtype this
    pub map_id: u64,    // TODO: newtype this
    pub map_type: WVWMap,
    pub coord: Option<Vec<f64>>, // TODO: newtype this?
    pub label_coord: Vec<f64>,   // TODO: newtype this?
    pub marker: Option<String>,
    pub chat_link: String,
    pub upgrade_id: Option<u64>, // TODO: NEwtype this?
}
