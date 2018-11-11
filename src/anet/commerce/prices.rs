use ::prelude::*;

gw2rs_id_u64!(PricesId);

/// `Prices` represent an overview of the associated item on the Trading Post. They represent
/// things like whether free accounts can list or buy the item, and the associated buy and sell
/// orders.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    id: PricesId,
    whitelisted: bool,
    buys: Pricing,
    sells: Pricing,
}

impl Prices {
    /// Returns the identifier of the item the `Prices` is for.
    pub fn id(&self) -> PricesId {
        self.id
    }

    /// Returns whether or not free accounts may buy or list this on the Trading Post.
    pub fn whitelisted(&self) -> bool {
        self.whitelisted
    }

    /// Returns the buy orders for the associated item on the Trading Post.
    pub fn buys(&self) -> &Pricing {
        &self.buys
    }

    /// Returns the sell orders for the associated item on the Trading Post.
    pub fn sells(&self) -> &Pricing {
        &self.sells
    }
}
