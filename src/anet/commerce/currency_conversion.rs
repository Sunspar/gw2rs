/// `CurrencyConversion`s represent data surrounding the conversion of gems and gold via the
/// Trading Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyConversion {
    pub coins_per_gem: u64,
    pub quantity: u64,
}
