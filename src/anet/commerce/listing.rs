use ::prelude::*;

gw2rs_id_u64!(ListingId);

/// `Listing`s represent trading post listing data for particular items.
#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    id: ListingId,
    buys: Vec<ListingItem>,
    sells: Vec<ListingItem>,
}

impl Listing {
    /// Retrieves the identifier associated with this `Listing`.
    pub fn id(&self) -> ListingId {
        self.id
    }

    /// Retrieves the listing items representing buy orders for this `Listing`.
    pub fn buys(&self) -> &[ListingItem] {
        &self.buys
    }

    /// Retrieves the listing items representing sell orders for this `Listing`.
    pub fn sells(&self) -> &[ListingItem] {
        &self.sells
    }
}
