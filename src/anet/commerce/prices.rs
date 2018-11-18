use ::prelude::*;

gw2rs_id_u64!(PricesId);

/// `Prices` represent an overview of the associated item on the Trading Post. They represent
/// things like whether free accounts can list or buy the item, and the associated buy and sell
/// orders.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id: PricesId,
    pub whitelisted: bool,
    pub buys: Pricing,
    pub sells: Pricing,
}
