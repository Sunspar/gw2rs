/// `AchievementTier`s represent a specific tier within an achievement.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementTier {
    pub count: u64,
    pub points: u64,
}

