use crate::prelude::*;

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
    id: ObjectiveId,
    name: String,
    #[serde(rename = "type")]
    objective_type: ObjectiveType,
    sector_id: u64, // TODO: Newtype this
    map_id: u64,    // TODO: newtype this
    map_type: WVWMap,
    coord: Option<Vec<f64>>, // TODO: newtype this?
    label_coord: Vec<f64>,   // TODO: newtype this?
    marker: Option<String>,
    chat_link: String,
    upgrade_id: Option<u64>, // TODO: NEwtype this?
}

impl Objective {
    /// Returns the Objective identifier used in querying various endpoints.
    pub fn id(&self) -> &ObjectiveId {
        &self.id
    }

    /// Returns this Objective's localized name, using the language assigned to the API wrapper when
    /// this Objective was initialized.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the associated objective "type" of this Objective.
    pub fn objective_type(&self) -> ObjectiveType {
        self.objective_type
    }

    /// Returns the Objective's map sector identification number.
    pub fn sector_id(&self) -> u64 {
        self.sector_id
    }

    /// The identification number of the map this Objective is found on.
    pub fn map_id(&self) -> u64 {
        self.map_id
    }

    /// The world versus world map this Objective is found on.
    pub fn map_type(&self) -> WVWMap {
        self.map_type
    }

    /// Optional coordinates of the Objective in the world.
    pub fn coord(&self) -> Option<&[f64]> {
        match self.coord {
            Some(ref v) => Some(v),
            None => None,
        }
    }

    /// Labelled coordinates of the Objective's map sector centroid in the world.
    pub fn label_coord(&self) -> &[f64] {
        &self.label_coord
    }

    /// The public URL of the image marker associated with this Objective.
    pub fn marker(&self) -> Option<&str> {
        match self.marker {
            Some(ref v) => Some(v),
            None => None,
        }
    }

    /// The chat link for this Objective.
    pub fn chat_link(&self) -> &str {
        &self.chat_link
    }

    /// The optional upgrade ID for this Objective.
    pub fn upgrade_id(&self) -> Option<u64> {
        self.upgrade_id
    }
}
