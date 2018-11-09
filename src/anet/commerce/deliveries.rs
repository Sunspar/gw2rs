use crate::prelude::*;

/// `TradingPostDeliveries` represents a user's items and gold waiting for pickup from the Trading
/// Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostDeliveries {
    coins: u64,
    items: Vec<TradingPostDeliveryItem>,
}

impl TradingPostDeliveries {
    /// Returns the number of coins waiting for pickup at the Trading Post.
    pub fn coins(&self) -> u64 {
        self.coins
    }

    /// Returns the delivery items waiting for pickup at the Trading Post.
    pub fn items(&self) -> &[TradingPostDeliveryItem] {
        &self.items
    }
}
