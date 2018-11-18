gw2rs_id_u64!(CurrencyId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Currency {
    pub id: CurrencyId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub order: u64,
}
