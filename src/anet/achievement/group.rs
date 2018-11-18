use ::anet::achievement::category::AchievementCategoryId;

gw2rs_id_string!(AchievementGroupId);

/// `AchievementGroup`s represent broad groupings of achievements in Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementGroup {
    pub id: AchievementGroupId,
    pub name: String,
    pub description: String,
    pub order: u64,
    pub categories: Vec<AchievementCategoryId>,
}
