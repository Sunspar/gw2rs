use ::prelude::*;

gw2rs_id_u64!(ListingId);

/// `Listing`s represent trading post listing data for particular items.
#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: ListingId,
    pub buys: Vec<ListingItem>,
    pub sells: Vec<ListingItem>,
}
