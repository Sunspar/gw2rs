gw2rs_id_u64!(TradingPostTransactionId);

/// `TradingPostTransaction`s represent historical or current buy or sell orders on the Trading
/// Post.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPostTransaction {
    id: TradingPostTransactionId,
    item_id: u64,
    price: u64,
    quantity: u64,
    created: String,
    purchased: Option<String>,
}

impl TradingPostTransaction {
    /// Returns the transaction's API identifier.
    pub fn id(&self) -> TradingPostTransactionId {
        self.id
    }

    /// Returns the API identifier for the associated item.
    pub fn item_id(&self) -> u64 {
        self.item_id
    }

    /// Returns the cost paid or recieved for this transaction.
    pub fn price(&self) -> u64 {
        self.price
    }

    /// Returns the quantity of item purchased/sold in this transaction.
    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    /// Returns the date as an ISO 8601 formatted String.
    pub fn created(&self) -> &str {
        &self.created
    }

    /// Returns the purchase date as an ISO 8601 formatted String.
    /// Returns an Option as some transactions which are still pending will not have an associated
    /// purchase date.
    pub fn purchased(&self) -> Option<&str> {
        match self.purchased {
            None => None,
            Some(ref v) => Some(v),
        }
    }
}
