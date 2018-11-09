/// `AchievementTier`s represent a specific tier within an achievement.
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementTier {
    count: u64,
    points: u64,
}

impl AchievementTier {
    /// Returns the "requirement amount" necessary to fulfill this tier of the achievement.
    /// E.g., the total number of kills for a kill achievement tier, the total amount of items in a
    /// collection achievement tier, etc.
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Returns the number of Achievement Points this tier is worth.
    pub fn points(&self) -> u64 {
        self.points
    }
}
