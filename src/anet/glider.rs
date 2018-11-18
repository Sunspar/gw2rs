use ::prelude::*;

gw2rs_id_u64!(GliderId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Glider {
    pub id: GliderId,
    pub unlock_items: Option<Vec<ItemId>>,
    pub order: u64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub default_dyes: Vec<ColorId>,
}
