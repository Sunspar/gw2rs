gw2rs_id_u64!(AchievementCategoryId);

use ::anet::achievement::achievement::AchievementId;

/// `AchievementCategory`s represent broad categories of achievements in Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementCategory {
    pub id: AchievementCategoryId,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub icon: String,
    pub achievements: Vec<AchievementId>,
}
