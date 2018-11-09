/// `CurrencyConversion`s represent data surrounding the conversion of gems and gold via the
/// Trading Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyConversion {
    coins_per_gem: u64,
    quantity: u64,
}

impl CurrencyConversion {
    /// Returns the number of coins needed per Gem at this conversion rate.
    pub fn coins_per_gem(&self) -> u64 {
        self.coins_per_gem
    }

    /// Returns the number of gems tradeable at this conversion rate.
    pub fn quantity(&self) -> u64 {
        self.quantity
    }
}
