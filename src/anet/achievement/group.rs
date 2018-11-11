use ::anet::achievement::category::AchievementCategoryId;

gw2rs_id_string!(AchievementGroupId);

/// `AchievementGroup`s represent broad groupings of achievements in Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementGroup {
    id: AchievementGroupId,
    name: String,
    description: String,
    order: u64,
    categories: Vec<AchievementCategoryId>,
}

impl AchievementGroup {
    /// Returns the achievement group's API identifier.
    pub fn id(&self) -> &AchievementGroupId {
        &self.id
    }

    /// Returns the achievement group's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the achievement group's description text.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns this achievement group's position in the group list it's a part of.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Returns a list of category identifiers that are part of this achievement group.
    pub fn categories(&self) -> &[AchievementCategoryId] {
        &self.categories
    }
}
