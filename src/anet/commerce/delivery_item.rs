gw2rs_id_u64!(TradingPostDeliveryItemId);

/// `TradingPostDeliveryItem`s represent items waiting for pickup from the Trading Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostDeliveryItem {
    id: TradingPostDeliveryItemId,
    count: u64,
}

impl TradingPostDeliveryItem {
    /// Returns the identifer of the item waiting for delivery at the Trading Post.`
    pub fn id(&self) -> TradingPostDeliveryItemId {
        self.id
    }

    /// Returns the number of the item waiting for delivery at the Trading Post.
    pub fn count(&self) -> u64 {
        self.count
    }
}
