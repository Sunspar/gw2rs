gw2rs_id_u64!(TradingPostDeliveryItemId);

/// `TradingPostDeliveryItem`s represent items waiting for pickup from the Trading Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostDeliveryItem {
    pub id: TradingPostDeliveryItemId,
    pub count: u64,
}
