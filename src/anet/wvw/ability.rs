gw2rs_id_u64!(WVWAbilityId);

#[derive(Debug, Deserialize)]
pub struct WVWAbility {
    id: WVWAbilityId,
    name: String,
    description: String,
    icon: String,
    ranks: Vec<WVWAbilityRank>,
}

impl WVWAbility {
    /// Returns the ability's internal identifier.
    pub fn id(&self) -> WVWAbilityId {
        self.id
    }

    /// Returns the localized name of this World versus World ability.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the localized description of this World versus World ability.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the URL of the in-game icon for this World versus World ability.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the set of upgrade tiers for this World versus World ability.
    pub fn ranks(&self) -> &[WVWAbilityRank] {
        self.ranks.as_slice()
    }
}

#[derive(Debug, Deserialize)]
pub struct WVWAbilityRank {
    cost: u64,
    effect: String,
}

impl WVWAbilityRank {
    /// Returns the cost of the tier unlock in WVW points.
    pub fn cost(&self) -> u64 {
        self.cost
    }

    /// Returns the localized description of what this tier of the ability offers.
    pub fn effect(&self) -> &str {
        &self.effect
    }
}
