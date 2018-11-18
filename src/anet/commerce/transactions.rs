gw2rs_id_u64!(TradingPostTransactionId);

/// `TradingPostTransaction`s represent historical or current buy or sell orders on the Trading
/// Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostTransaction {
    pub id: TradingPostTransactionId,
    pub item_id: u64,
    pub price: u64,
    pub quantity: u64,
    pub created: String,
    pub purchased: Option<String>,
}
