/// `ListingItem`s represent an individual instance of a Trading Post buy or sell order for the
/// associated item. They are always tied to a `Listing`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListingItem {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}
