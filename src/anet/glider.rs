use crate::prelude::*;

gw2rs_id_u64!(GliderId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Glider {
    id: GliderId,
    unlock_items: Option<Vec<ItemId>>,
    order: u64,
    icon: String,
    name: String,
    description: String,
    default_dyes: Vec<ColorId>,
}

impl Glider {
    /// Returns the glider's internal identifier.
    pub fn id(&self) -> GliderId {
        self.id
    }

    /// Returns the set of items used to unlock the glider.
    pub fn unlock_items(&self) -> Option<&[ItemId]> {
        match self.unlock_items {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }

    /// Returns the ordering of this glider in the list of gliders within the in-gane UI.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Returns the url to the glider's icon in-gane.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the glider's localized name as displayed in-game.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the glider's localized description as displayed in-game.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the set of default dyes applied to this glider on acquire.
    pub fn default_dyes(&self) -> &[ColorId] {
        self.default_dyes.as_slice()
    }
}
