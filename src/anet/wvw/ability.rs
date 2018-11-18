gw2rs_id_u64!(WVWAbilityId);

#[derive(Debug, Deserialize)]
pub struct WVWAbility {
    pub id: WVWAbilityId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub ranks: Vec<WVWAbilityRank>,
}

#[derive(Debug, Deserialize)]
pub struct WVWAbilityRank {
    pub cost: u64,
    pub effect: String,
}
