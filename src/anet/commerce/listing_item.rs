/// `ListingItem`s represent an individual instance of a Trading Post buy or sell order for the
/// associated item. They are always tied to a `Listing`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListingItem {
    listings: u64,
    unit_price: u64,
    quantity: u64,
}

impl ListingItem {
    /// Returns the number of individual listings the item has on the Trading Post.
    /// (e.g. Two players selling the same item at the same price will show up in the same
    /// `ListingItem`.)
    pub fn listings(&self) -> u64 {
        self.listings
    }

    /// Returns the unit price of the item on the Trading Post.
    pub fn unit_price(&self) -> u64 {
        self.unit_price
    }

    /// Returns the quantity of the item on the Trading Post.
    pub fn quantity(&self) -> u64 {
        self.quantity
    }
}
