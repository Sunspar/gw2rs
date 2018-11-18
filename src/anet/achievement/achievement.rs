use ::prelude::*;

gw2rs_id_u64!(AchievementId);

/// `Achievement`s represent achievements within Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: AchievementId,
    pub name: String,
    pub description: String,
    pub requirement: String,
    pub locked_text: String,
    #[serde(rename = "type")]
    pub achievement_type: String, // TODO: Enum this?
    pub flags: Vec<String>, // TODO: Enum this?
    pub tiers: Vec<AchievementTier>,
    pub rewards: Option<Vec<AchievementReward>>,
}
