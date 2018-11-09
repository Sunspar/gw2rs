use crate::prelude::*;

gw2rs_id_u64!(AchievementId);

/// `Achievement`s represent achievements within Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    id: AchievementId,
    name: String,
    description: String,
    requirement: String,
    locked_text: String,
    #[serde(rename = "type")]
    achievement_type: String, // TODO: Enum this?
    flags: Vec<String>, // TODO: Enum this?
    tiers: Vec<AchievementTier>,
    rewards: Option<Vec<AchievementReward>>,
}

impl Achievement {
    /// Returns the achievement's internal identifier.
    pub fn id(&self) -> AchievementId {
        self.id
    }

    /// Returns the achievement's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the achievement's description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the achievement's requirement as listed in-game.
    pub fn requirement(&self) -> &str {
        &self.requirement
    }

    /// Returns the text that appears when the achievement's progression is locked.
    pub fn locked_text(&self) -> &str {
        &self.locked_text
    }

    /// Returns the achievement's achievement type.
    pub fn achievement_type(&self) -> &str {
        &self.achievement_type
    }

    /// Returns the set of flags associated with this achievement.
    pub fn flags(&self) -> &[String] {
        &self.flags
    }

    /// Returns the tier information associated with the achievement.
    pub fn tiers(&self) -> &[AchievementTier] {
        &self.tiers
    }

    /// Returns the rewards associated with the achievement.
    pub fn rewards(&self) -> Option<&[AchievementReward]> {
        match self.rewards {
            None => None,
            Some(ref v) => Some(v.as_slice()),
        }
    }
}
