gw2rs_id_u64!(AchievementCategoryId);

use crate::anet::achievement::achievement::AchievementId;

/// `AchievementCategory`s represent broad categories of achievements in Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategory {
    id: AchievementCategoryId,
    name: String,
    description: String,
    order: u64,
    icon: String,
    achievements: Vec<AchievementId>,
}

impl AchievementCategory {
    /// Retrieves the achievement category API identifier.
    pub fn id(&self) -> AchievementCategoryId {
        self.id
    }

    /// Retrieves the name of this achievement category.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieves the description of this achievement category.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Retrieves the position of this achievement category in the Achievements UI in Guild
    /// Wars 2.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Retrieves the category's public icon url.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Retrieves a set of achievement identifiers for achievements within this category.
    pub fn achievements(&self) -> &[AchievementId] {
        &self.achievements
    }
}
