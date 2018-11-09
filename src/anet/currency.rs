gw2rs_id_u64!(CurrencyId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Currency {
    id: CurrencyId,
    name: String,
    description: String,
    icon: String,
    order: u64,
}

impl Currency {
    /// Returns the Currency's internal identifier.
    pub fn id(&self) -> CurrencyId {
        self.id
    }

    /// Returns the Currency's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Currency's localized description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns a url to the currency's in-game image.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the order of this currency in the in-game UI. Lower orders appear higher in the
    /// currency tab's list.
    pub fn order(&self) -> u64 {
        self.order
    }
}
