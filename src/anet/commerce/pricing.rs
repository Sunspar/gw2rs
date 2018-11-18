/// `Pricing`s represent an item's pricing data on the Trading Post. More specifically, they
/// represent the quantity and unit price of a particular buy or sell order placed for the
/// associated item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    pub quantity: u64,
    pub unit_price: u64,
}
