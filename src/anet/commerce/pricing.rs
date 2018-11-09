/// `Pricing`s represent an item's pricing data on the Trading Post. More specifically, they
/// represent the quantity and unit price of a particular buy or sell order placed for the
/// associated item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    quantity: u64,
    unit_price: u64,
}

impl Pricing {
    /// Returns the quantity of the item being sold in this `Pricing`.
    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    /// Returns the unit cost of the item being sold in this `Pricing`.
    pub fn unit_price(&self) -> u64 {
        self.unit_price
    }
}
