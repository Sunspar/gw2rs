gw2rs_id_u64!(CatId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Cat {
    id: CatId,
    hint: String,
}

impl Cat {
    /// Returns the cat's internal identifier.
    pub fn id(&self) -> CatId {
        self.id
    }

    /// Returns the localized text hint for getting this cat.
    pub fn hint(&self) -> &str {
        &self.hint
    }
}
