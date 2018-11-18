use ::prelude::*;

/// `TradingPostDeliveries` represents a user's items and gold waiting for pickup from the Trading
/// Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostDeliveries {
    pub coins: u64,
    pub items: Vec<TradingPostDeliveryItem>,
}
